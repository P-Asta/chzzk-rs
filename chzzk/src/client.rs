use crate::{
    channel::{ChannelId, ChannelLiveStatus, ChatAccessToken, ChatChannelId},
    error::{chain_error, Error},
    user::User,
    Response,
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

    /// Send request to given URL.
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails, or response is not valid json.
    async fn do_request<'a, T>(&self, full_path: String) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        println!("request to: {full_path}.");
        let mut request = self.client.get(full_path);

        if let Some(nid) = &self.nid {
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

    async fn request_chzzk<'a, T>(&self, path: String) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.do_request(format!("https://api.chzzk.naver.com/{path}"))
            .await
    }

    async fn request_game<'a, T>(&self, path: String) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.do_request(format!("https://comm-api.game.naver.com/nng_main/{path}"))
            .await
    }

    /// Get channel live status of channel with given id.
    /// ex) https://api.chzzk.naver.com/polling/v2/channels/1dac6492f81d89e261f692bb6b79ff57/live-status
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn get_channel_live_status(
        &self,
        channel_id: &ChannelId,
    ) -> Result<ChannelLiveStatus, Error> {
        let response_object = self
            .request_chzzk::<Response<ChannelLiveStatus>>(format!(
                "polling/v2/channels/{}/live-status",
                *channel_id
            ))
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
        let response_object = self
            .request_game::<Response<User>>("v1/user/getUserStatus".into())
            .await?;

        Ok(response_object.content)
    }

    /// Get AccessToken.
    /// ex) https://comm-api.game.naver.com/nng_main/v1/chats/access-token?channelId=N1SMvY&chatType=STREAMING
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_access_token(&self, chat_id: &ChatChannelId) -> Result<ChatAccessToken, Error> {
        let response_object = self
            .request_game::<Response<ChatAccessToken>>(format!(
                "v1/chats/access-token?channelId={}&chatType=STREAMING",
                chat_id
            ))
            .await?;

        if response_object.code == 42601 {
            return Err("Broadcast is age-restricted, nidAuth, nidSession is required".into());
        }

        Ok(response_object.content)
    }
}

impl Default for ChzzkClient {
    fn default() -> Self {
        Self::new()
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
