use crate::{channel::{ChannelId, ChatChannelId}, debug_println};
use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use serde_json::Value;
use std::{
    future::Future,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::{
    super::{
        client::ChzzkClient,
        error::{chain_error, Error},
        r#macro::{jsonvalue_unwrap_or_return, simple_get, simple_get_as},
    },
    handler_vec::*,
    types::*,
};

type WriteStream = SplitSink<
    WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;
type ReadStream =
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>;

#[derive(Clone)]
pub struct ChatClient {
    client: ChzzkClient,
    channel_id: ChannelId,
    inner: Arc<Inner>,
}

struct Inner {
    write_stream: Mutex<Option<WriteStream>>,
    sid: Mutex<Option<String>>,
    chat_id: Mutex<Option<ChatChannelId>>,
    event_handlers: RwLock<EventHandlerCollection>,
}

struct EventHandlerCollection {
    chat: HandlerVec<ChatEvent>,
}

impl ChatClient {
    pub fn new(client: ChzzkClient, channel_id: &ChannelId) -> Self {
        Self {
            client,
            channel_id: channel_id.clone(),
            inner: Arc::new(Inner {
                write_stream: Mutex::new(None),
                sid: Mutex::new(None),
                chat_id: Mutex::new(None),
                event_handlers: RwLock::new(EventHandlerCollection {
                    chat: HandlerVec::new(),
                }),
            }),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        if self.inner.write_stream.lock().await.is_some() {
            // already connected
            return Err("chat.connect: already_connected".into());
        }

        // Get ChatID
        let channel_status = self
            .client
            .get_channel_live_status(&self.channel_id)
            .await
            .map_err(chain_error!("chat.connect: live_channel_status error"))?;
        let chat_id = channel_status.chat_channel_id;

        *self.inner.chat_id.lock().await = Some(chat_id.clone());

        // Get UID
        let user = self.client.get_user_status().await.map_err(chain_error!(
            "chat.connect: get_user_status error. maybe wrong auth information"
        ))?;

        // Get accTkn
        let chat_status = self
            .client
            .get_access_token(&chat_id)
            .await
            .map_err(chain_error!("chat.connect: get_access_token error"))?;

        // Connect to Websocket
        let ss_id = rand::random::<u32>() % 10 + 1; // Load Balancing
        let addr = format!("wss://kr-ss{}.chat.naver.com/chat", ss_id);
        let (stream, _response) = tokio_tungstenite::connect_async(addr)
            .await
            .map_err(chain_error!("chat.connect: websocket connect failed"))?;
        let (write, read) = stream.split();

        // Store in self
        *self.inner.write_stream.lock().await = Some(write);
        debug_println!("Response: {}", response.status(),);

        // Run handler
        tokio::spawn(ChatClient::response_handler(read, self.clone()));
        tokio::spawn(ChatClient::poll(self.clone()));
        tokio::spawn(ChatClient::ping(self.clone()));

        // Prepare first message
        let payload = Message::from(
            serde_json::json!({
                "bdy": serde_json::json!({
                    "accTkn": chat_status.access_token,
                    "auth": "SEND",
                    "devType": 2001,
                    "uid": user.user_id_hash.0,
                }),
                "cmd": ChatCommand::Connect as i32,
                "tid": 1,
                "cid": chat_id.as_str(),
                "svcid": "game",
                "ver": "3",
            })
            .to_string(),
        );

        self.send_message(payload).await.unwrap();

        while self.inner.sid.lock().await.is_none() {
            // spin until ack comes. empirically it spinned five times.
            tokio::time::sleep(Duration::from_millis(1)).await;
            // todo! timeout
        }

        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.inner.write_stream.lock().await.take();
        self.inner.chat_id.lock().await.take();
        self.inner.sid.lock().await.take();
    }

    pub async fn send_chat(&self, message: &str) -> Result<(), Error> {
        if self.inner.write_stream.lock().await.is_none() {
            return Err("not connected".into());
        }

        let unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let sid_lock = self.inner.sid.lock().await;
        let sid = sid_lock.as_ref().unwrap().as_str();
        let chat_id_lock = self.inner.chat_id.lock().await;
        let chat_id = chat_id_lock.as_ref().unwrap().as_str();

        let chat_msg = Message::from(
            serde_json::json!({
                "bdy": {
                    "extras": serde_json::json!({
                        "chatType": "STREAMING",
                        "osType": "PC",
                        // "extraToken": extra_token,
                        "streamingChannelId": self.channel_id.clone(),
                        "emojis": serde_json::json!({}),
                    }).to_string().as_str(),
                    "msg": message,
                    "msgTime": unix_time,
                    "msgTypeCode": ChatType::Text as i32
                },
                "retry": false,
                "cmd": ChatCommand::SendChat as i32,
                "sid": sid,
                "tid": 2,
                "cid": chat_id,
                "svcid": "game",
                "ver": "3",
            })
            .to_string(),
        );

        drop(sid_lock);
        drop(chat_id_lock);

        self.send_message(chat_msg).await.unwrap();
        Ok(())
    }

    /// Private method to send a message to chzzk chat server.
    ///
    /// # Errors
    ///
    /// This function will return an error if send fails.
    async fn send_message(&self, message: Message) -> Result<(), Error> {
        debug_println!("Sent {message}");
        match &mut *self.inner.write_stream.lock().await {
            Some(s) => s.send(message).await.map_err(chain_error!("send failed")),
            None => Err("Not connected".into()),
        }
    }

    async fn response_handler(mut read_stream: ReadStream, mut chat: ChatClient) {
        debug_println!("handler runs");
        while chat.inner.write_stream.lock().await.is_some() {
            if let Err(err) = ChatClient::do_handle(&mut read_stream, &mut chat).await {
                debug_println!("event_handler caught error: {err}");
                if err.0 == "websocket disconnected." {
                    // yeah, this looks bit silly.
                    break;
                }
            }
        }
    }

    async fn do_handle(read_stream: &mut ReadStream, client: &mut ChatClient) -> Result<(), Error> {
        let message = read_stream
            .next() // read
            .await
            .ok_or("None in event handler")? // next() returned None
            .map_err(chain_error!("websocket disconnected")); // next() returned Err. disconnected

        if let Err(err) = message {
            client.disconnect().await;
            return Err(err);
        }

        let text = message
            .unwrap()
            .into_text()
            .map_err(chain_error!("do_handle: message is not a text"))?; // message is not text

        debug_println!("Recieved {text}");

        let json = serde_json::from_str::<Value>(text.as_str())
            .map_err(chain_error!("do_handle: message is not a json."))?;
        // {
        //     json::JsonValue::Object(object) => object,
        //     not_object => Err(format!("Not an object {}", not_object))?,
        // };

        let cmd = ChatCommand::try_from(simple_get_as!(json, "cmd", as_i64)?)
            .or(Err("Wrong command."))?;

        let body = simple_get!(json, "bdy")?;

        match cmd {
            ChatCommand::Ping => todo!(),
            ChatCommand::Pong => {}
            ChatCommand::Connect => todo!(),
            ChatCommand::Connected => {
                let body = jsonvalue_unwrap_or_return!(Value::Object, body)
                    .map_err(chain_error!("do_handle.connected"))?;
                let sid = simple_get_as!(body, "sid", as_str)?;
                *client.inner.sid.lock().await = Some(sid.into());

                // todo!()
            }
            ChatCommand::RequestRecentChat => todo!(),
            ChatCommand::RecentChat => {}
            ChatCommand::Event => todo!(),
            ChatCommand::Chat => {
                let chats = jsonvalue_unwrap_or_return!(Value::Array, body)
                    .map_err(chain_error!("do_handle.chat"))?;
                ChatClient::handle_chat(client, chats).await?;
            }
            ChatCommand::Donation => todo!(),
            ChatCommand::Kick => todo!(),
            ChatCommand::Block => todo!(),
            ChatCommand::Blind => {}
            ChatCommand::Notice => todo!(),
            ChatCommand::Penalty => todo!(),
            ChatCommand::SendChat => todo!(),
        }

        Ok(())
    }

    async fn handle_chat(client: &ChatClient, chats: &[Value]) -> Result<(), Error> {
        for chat in chats {
            let chat_event = serde_json::from_value::<ChatEvent>(chat.clone())
                .map_err(chain_error!("do_handle.chat"))?;

            client
                .inner
                .event_handlers
                .read()
                .await
                .chat
                .broadcast(chat_event)
                .await;
        }

        Ok(())
    }

    async fn poll(client: ChatClient) {
        while client.inner.write_stream.lock().await.is_some() {
            tokio::time::sleep(Duration::from_secs(60)).await;

            match ChatClient::do_poll(&client.client, &client.channel_id).await {
                Ok(chat_id) => *client.inner.chat_id.lock().await = Some(chat_id.clone()),
                Err(_err) => {
                    debug_println!("poll error: {:?}", err);
                    // chat.disconnect();
                    break;
                }
            }
        }
    }

    async fn do_poll(client: &ChzzkClient, channel_id: &ChannelId) -> Result<ChatChannelId, Error> {
        let channel_status = client.get_channel_live_status(channel_id).await;
        Ok(channel_status
            .map_err(chain_error!("poll: live_channel_status error"))?
            .open_or("poll: not livestreaming")?
            .chat_channel_id)
    }

    async fn ping(client: ChatClient) {
        let ping_object = Message::from(
            serde_json::json!({
                "cmd": ChatCommand::Ping as i32,
                "ver": "2"
            })
            .to_string(),
        );

        while client.inner.write_stream.lock().await.is_some() {
            tokio::time::sleep(Duration::from_secs(20)).await;
            let _ = client.send_message(ping_object.clone()).await;
        }
    }

    pub async fn register_on_chat<F, Fut>(&self, f: F)
    where
        F: FnOnce(Arc<ChatEvent>) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let ff = HandlerHolder { handler: f };
        self.inner
            .event_handlers
            .write()
            .await
            .chat
            .0
            .push(Box::new(ff))
    }
}

// let req = Message::from(
//     serde_json::json! {
//         bdy: serde_json::json! {
//             recentMessageCount: 1
//         },
//         cmd: ChatCommand::RequestRecentChat as i32,
//         sid: sid,
//         tid: 2,
//         cid: chat_id,
//         svcid: "game",
//         ver: "3",
//     }
//     .to_string(),
// );
