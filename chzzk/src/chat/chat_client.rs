use std::{
    future::Future,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use json::JsonValue;
use tokio::sync::Mutex;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::user::UserIdHash;

use super::{
    super::{
        error::{chain_error, Error},
        client::ChzzkClient,
        r#macro::{jsonvalue_unwrap_or_return, simple_get, simple_get_as},
    },
    chat_handle::*,
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
    channel_id: String,
    write_stream: Arc<Mutex<Option<WriteStream>>>,
    sid: Arc<Mutex<Option<String>>>,
    chat_id: Arc<Mutex<Option<String>>>,
    event_handlers: Arc<Mutex<EventHandlerCollection>>,
}

struct EventHandlerCollection {
    chat: HandlerVec<ChatEvent>,
}

impl ChatClient {
    pub fn new(client: ChzzkClient, channel_id: &str) -> Self {
        Self {
            client,
            channel_id: channel_id.to_string(),
            write_stream: Arc::new(Mutex::new(None)),
            sid: Arc::new(Mutex::new(None)),
            chat_id: Arc::new(Mutex::new(None)),
            event_handlers: Arc::new(Mutex::new(EventHandlerCollection {
                chat: HandlerVec::new(),
            })),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        if self.write_stream.lock().await.is_some() {
            // already connected
            return Err("chat.connect: already_connected".into());
        }

        // Get ChatID
        let channel_status = self
            .client
            .get_channel_live_status(self.channel_id.as_str())
            .await
            .map_err(chain_error!("chat.connect: live_channel_status error"))?;
        let chat_id = channel_status.chat_channel_id;

        *self.chat_id.lock().await = Some(chat_id.clone());

        // Get UID
        let user = self.client.get_user_status().await.map_err(chain_error!(
            "chat.connect: get_user_status error. maybe wrong auth information"
        ))?;

        // Get accTkn
        let chat_status = self
            .client
            .get_access_token(chat_id.as_str())
            .await
            .map_err(chain_error!("chat.connect: get_access_token error"))?;

        // Connect to Websocket
        let ss_id = rand::random::<u32>() % 10 + 1; // Load Balancing
        let addr = format!("wss://kr-ss{}.chat.naver.com/chat", ss_id);
        let (stream, response) = tokio_tungstenite::connect_async(addr)
            .await
            .map_err(chain_error!("chat.connect: websocket connect failed"))?;
        let (write, read) = stream.split();

        // Store in self
        *self.write_stream.lock().await = Some(write);
        println!("Response: {}", response.status(),);

        // Run handler
        tokio::spawn(ChatClient::response_handler(read, self.clone()));
        tokio::spawn(ChatClient::poll(self.clone()));

        // Prepare first message
        let payload = Message::from(
            json::object! {
                bdy: json::object! {
                    accTkn: chat_status.access_token,
                    auth: "SEND",
                    devType: 2001,
                    uid: user.user_id_hash.0,
                },
                cmd: ChatCommand::Connect as i32,
                tid: 1,
                cid: chat_id.as_str(),
                svcid: "game",
                ver: "3",
            }
            .to_string(),
        );

        self.send_message(payload).await.unwrap();

        while self.sid.lock().await.is_none() {
            // spin until ack comes. empirically it spinned five times.
            tokio::time::sleep(Duration::from_millis(1)).await;
            // todo! timeout
        }

        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.write_stream.lock().await.take();
        self.chat_id.lock().await.take();
        self.sid.lock().await.take();
    }

    pub async fn send_chat(&self, message: &str) -> Result<(), Error> {
        if self.write_stream.lock().await.is_none() {
            return Err("not connected".into());
        }

        let unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let sid_lock = self.sid.lock().await;
        let sid = sid_lock.as_ref().unwrap().as_str();
        let chat_id_lock = self.chat_id.lock().await;
        let chat_id = chat_id_lock.as_ref().unwrap().as_str();

        let chat_msg = Message::from(
            json::object! {
                bdy: {
                    extras: json::object! {
                        chatType: "STREAMING",
                        osType: "PC",
                        // extraToken: extra_token,
                        streamingChannelId: self.channel_id.clone(),
                        emojis: json::object! {},
                    }.to_string().as_str(),
                    msg: message,
                    msgTime: unix_time,
                    msgTypeCode: ChatType::Text as i32
                },
                retry: false,
                cmd: ChatCommand::SendChat as i32,
                sid: sid,
                tid: 2,
                cid: chat_id,
                svcid: "game",
                ver: "3",
            }
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
        println!("Sent {message}");
        match &mut *self.write_stream.lock().await {
            Some(s) => s.send(message).await.map_err(chain_error!("send failed")),
            None => Err("Not connected".into()),
        }
    }

    async fn response_handler(mut read_stream: ReadStream, mut chat: ChatClient) {
        println!("handler runs");
        while chat.write_stream.lock().await.is_some() {
            if let Err(err) = ChatClient::do_handle(&mut read_stream, &mut chat).await {
                println!("event_handler caught error: {err}");
                if err.0 == "websocket disconnected." {
                    // yeah, this looks bit silly.
                    break;
                }
            }
        }
    }

    async fn do_handle(read_stream: &mut ReadStream, chat: &mut ChatClient) -> Result<(), Error> {
        let message = read_stream
            .next()
            .await
            .ok_or("None in event handler")? // next() returned None
            .map_err(chain_error!("websocket disconnected")); // next() returned Err. disconnected

        if let Err(err) = message {
            chat.disconnect().await;
            return Err(err);
        }

        let message = message
            .unwrap()
            .into_text()
            .map_err(chain_error!("do_handle: message is not a text"))?; // message is not text

        println!("Recieved {message}");

        let json = match json::parse(message.as_str())
            .map_err(chain_error!("do_handle: message is not a json."))?
        {
            json::JsonValue::Object(object) => object,
            not_object => Err(format!("Not an object {}", not_object))?,
        };

        let cmd = ChatCommand::try_from(simple_get_as!(json, "cmd", as_i32)?)
            .or(Err("Wrong command."))?;

        let body = simple_get!(json, "bdy")?;

        match cmd {
            ChatCommand::Ping => todo!(),
            ChatCommand::Pong => todo!(),
            ChatCommand::Connect => todo!(),
            ChatCommand::Connected => {
                let body = jsonvalue_unwrap_or_return!(JsonValue::Object, body)
                    .map_err(chain_error!("do_handle.connected"))?;
                let sid = simple_get_as!(body, "sid", as_str)?;
                *chat.sid.lock().await = Some(sid.into());

                // todo!()
            }
            ChatCommand::RequestRecentChat => todo!(),
            ChatCommand::RecentChat => {}
            ChatCommand::Event => todo!(),
            ChatCommand::Chat => {
                let body = jsonvalue_unwrap_or_return!(JsonValue::Array, body)
                    .map_err(chain_error!("do_handle.chat"))?;
                let body = jsonvalue_unwrap_or_return!(JsonValue::Object, &body[0])
                    .map_err(chain_error!("do_handle.chat"))?;

                let time =
                    UNIX_EPOCH + Duration::from_millis(simple_get_as!(body, "ctime", as_u64)?);
                let message = simple_get_as!(body, "msg", as_str)?.to_string();
                let user = UserIdHash(simple_get_as!(body, "uid", as_str)?.to_string());

                chat.event_handlers
                    .lock()
                    .await
                    .chat
                    .broadcast(ChatEvent {
                        time,
                        message,
                        user,
                    })
                    .await;
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

    async fn poll(chat: ChatClient) {
        while chat.write_stream.lock().await.is_some() {
            match ChatClient::do_poll(&chat.client, chat.channel_id.as_str()).await {
                Ok(chat_id) => *chat.chat_id.lock().await = Some(chat_id.clone()),
                Err(err) => {
                    println!("poll error: {:?}", err);
                    // chat.disconnect();
                    break;
                }
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    async fn do_poll(client: &ChzzkClient, channel_id: &str) -> Result<String, Error> {
        let channel_status = client.get_channel_live_status(channel_id).await;
        Ok(channel_status
            .map_err(chain_error!("poll: live_channel_status error"))?
            .open_or("poll: not livestreaming")?
            .chat_channel_id)
    }

    pub async fn register_on_chat<F, Fut>(&self, f: F)
    where
        F: FnOnce(ChatEvent) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let ff = HandlerHolder { handler: f };
        self.event_handlers.lock().await.chat.0.push(Box::new(ff))
    }
}

// #[tokio::test]
// async fn test_chat() {
//     use crate::env::{AUT, SES};
//     let mut c = ChzzkClient::new();
//     c.sign_in(AUT, SES);
//     let mut chatter = ChatClient::new(c, "1dac6492f81d89e261f692bb6b79ff57"); // ray_remi 1dac6492f81d89e261f692bb6b79ff57
//     chatter.connect().await.unwrap();
//     chatter.send_chat("test send_chat").await.unwrap();

//     // let req = Message::from(
//     //     json::object! {
//     //         bdy: json::object! {
//     //             recentMessageCount: 1
//     //         },
//     //         cmd: ChatCommand::RequestRecentChat as i32,
//     //         sid: sid,
//     //         tid: 2,
//     //         cid: chat_id,
//     //         svcid: "game",
//     //         ver: "3",
//     //     }
//     //     .to_string(),
//     // );
// }
