use crate::{
    channel::{Channel, ChannelId, ChannelLiveStatus, ChatAccessToken, ChatChannelId}, debug_println, error::{chain_error, Error}, user::User, Response
};

#[derive(Clone)]
struct Nid {
    pub aut: String,
    pub ses: String,
}

#[derive(Clone)]
pub struct ChzzkClient {
    nid: Option<Nid>,
    client: reqwest::Client,
}

#[derive(serde::Deserialize)]
struct DropResponse {}

impl ChzzkClient {
    pub fn new() -> Self {
        let c = reqwest::Client::new();
        Self {
            client: c,
            nid: None,
        }
    }

    pub fn new_with_sign_in(aut: &str, ses: &str) -> Self {
        let mut x = Self::new();
        x.sign_in(aut, ses);
        x
    }

    pub fn sign_in(&mut self, aut: &str, ses: &str) {
        let nid = Nid {
            aut: aut.into(),
            ses: ses.into(),
        };
        self.nid = Some(nid);
    }

    fn chzzk(path: &str) -> ChzzkRequestBuilder {
        ChzzkRequestBuilder::new(format!("https://api.chzzk.naver.com/{path}"))
    }

    fn game(path: &str) -> ChzzkRequestBuilder {
        ChzzkRequestBuilder::new(format!("https://comm-api.game.naver.com/nng_main/{path}"))
    }

    /// Get channel live status of channel with given id.
    /// ex) https://api.chzzk.naver.com/polling/v2/channels/1dac6492f81d89e261f692bb6b79ff57/live-status
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_channel_live_status(
        &self,
        channel_id: &ChannelId,
    ) -> Result<ChannelLiveStatus, Error> {
        let response_object =
            Self::chzzk(format!("polling/v2/channels/{}/live-status", **channel_id).as_str())
                .get()
                .send::<Response<ChannelLiveStatus>>(self)
                .await?;

        Ok(response_object.content)
    }

    /// Get user status of currently logged-in user (bot).
    /// ex) https://comm-api.game.naver.com/nng_main/v1/user/getUserStatus
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_user_status(&self) -> Result<User, Error> {
        let response_object = Self::game("v1/user/getUserStatus")
            .get()
            .send::<Response<User>>(self)
            .await?;
        Ok(response_object.content)
    }

    /// Get AccessToken.
    /// ex) https://comm-api.game.naver.com/nng_main/v1/chats/access-token?channelId=N1SMvY&chatType=STREAMING
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_access_token(
        &self,
        chat_id: &ChatChannelId,
    ) -> Result<ChatAccessToken, Error> {
        let response_object = Self::game(
            format!(
                "v1/chats/access-token?channelId={}&chatType=STREAMING",
                **chat_id
            )
            .as_str(),
        )
        .get()
        .send::<Response<ChatAccessToken>>(self)
        .await?;

        if response_object.code == 42601 {
            return Err("Broadcast is age-restricted, nidAuth, nidSession is required".into());
        }

        Ok(response_object.content)
    }

    /// Get channel information.
    /// ex) https://api.chzzk.naver.com/service/v1/channels/9ae7d38b629b78f48e49fb3106218ff5
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_channel_info(&self, channel_id: &ChannelId) -> Result<Channel, Error> {
        let response = Self::chzzk(format!("service/v1/channels/{}", **channel_id).as_str())
            .get()
            .send::<Response<Channel>>(self)
            .await?;

        Ok(response.content)
    }

    pub async fn set_chat_notice(
        &self,
        channel_id: &ChannelId,
        chat_id: Option<&ChatChannelId>,
        sender: &ChannelId,
        time: u64,
    ) -> Result<(), Error> {
        let chat_id = match chat_id {
            Some(x) => x,
            None => {
                &self
                    .get_channel_live_status(channel_id)
                    .await?
                    .chat_channel_id
            }
        };

        let payload = serde_json::json!({
            "channelId": chat_id,
            "chatType": "STREAMING",
            // "extras": "{\"chatType\":\"STREAMING\",\"osType\":\"PC\",\"streamingChannelId\":\"1dac6492f81d89e261f692bb6b79ff57\",\"emojis\":{}}",
            "message": "0",
            "messageTime": time,
            "messageUserIdHash": sender,
            "streamingChannelId": channel_id,
        });

        Self::game("v1/chats/notices")
            .post(Some(payload.to_string()))
            .send::<DropResponse>(self)
            .await?;
        Ok(())
    }
}

impl Default for ChzzkClient {
    fn default() -> Self {
        Self::new()
    }
}

struct ChzzkRequestBuilder {
    url: String,
    is_method_post: bool,
    body: Option<String>,
}

impl ChzzkRequestBuilder {
    pub fn new(url: String) -> Self {
        Self {
            url,
            is_method_post: false,
            body: None,
        }
    }

    pub fn get(self) -> Self {
        self
    }

    pub fn post(self, body: Option<String>) -> Self {
        Self {
            url: self.url,
            is_method_post: true,
            body,
        }
    }

    async fn send<T>(self, client: &ChzzkClient) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        debug_println!("request to: {}.", self.url);
        let mut request = if self.is_method_post {
            client
                .client
                .post(self.url)
                .header("Content-Type", "application/json")
                .body(self.body.unwrap_or_default())
        } else {
            client.client.get(self.url)
        };

        if let Some(nid) = &client.nid {
            request = request.header("Cookie", format!("NID_AUT={};NID_SES={}", nid.aut, nid.ses));
        }

        let response = request
            .send()
            .await
            .map_err(chain_error!("do_request: failed to get response"))?;

        let text = response
            .text()
            .await
            .map_err(chain_error!("do_request: response is not a text"))?;

        let json = serde_json::from_str::<T>(&text)
            // let json = json::parse(text.as_str())
            .map_err(chain_error!(format!(
                "do_request: response is not a json. {}",
                text
            )))?;

        Ok(json)
    }
}

// #[tokio::test]
// pub async fn test_live() {
//     use crate::env::{AUT, SES};
//     let b = ChzzkClient::new_with_sign_in(AUT, SES); // get it from your cookie!
//     let channel_id = "1dac6492f81d89e261f692bb6b79ff57";
//     b.get_channel_live_status(channel_id)
//         .await
//         .unwrap();
// }
