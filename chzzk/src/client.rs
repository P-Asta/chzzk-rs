use crate::{
    authorization::AccessTokenResponse, error::Error, request_builder::ChzzkRequestBuilder,
    Response,
};

#[cfg(feature = "unofficial")]
use crate::{
    channel::ChannelId,
    request_builder::Nid,
    unofficial::{
        channel::{Channel, ChannelLiveStatus, ChatAccessToken, ChatChannelId},
        search_channel::ChannelSearchBundle,
        search_live::LiveSearchBundle,
        search_video::VideoSearchBundle,
        user::User,
        SearchContent,
    },
};

#[derive(Clone)]
pub struct ChzzkClient {
    #[cfg(feature = "unofficial")]
    pub(super) nid: Option<Nid>,
    pub(super) client: reqwest::Client,
    pub(super) client_id: Option<String>,
    pub(super) client_secret: Option<String>,
    pub(super) authorization_code: Option<String>,
}

#[derive(serde::Deserialize)]
struct DropResponse {}

impl ChzzkClient {
    pub fn new() -> Self {
        let c = reqwest::Client::new();
        #[cfg(feature = "unofficial")]
        return Self {
            client: c,
            nid: None,
            client_id: None,
            client_secret: None,
            authorization_code: None,
        };

        #[cfg(not(feature = "unofficial"))]
        Self {
            client: c,
            client_id: None,
            client_secret: None,
            authorization_code: None,
        }
    }

    #[cfg(feature = "unofficial")]
    pub fn new_with_sign_in(aut: &str, ses: &str) -> Self {
        let mut x = Self::new();
        x.sign_in(aut, ses);
        x
    }

    #[cfg(feature = "unofficial")]
    pub fn sign_in(&mut self, aut: &str, ses: &str) {
        let nid = Nid {
            aut: aut.into(),
            ses: ses.into(),
        };
        self.nid = Some(nid);
    }

    pub async fn access_token(&self) -> Result<AccessTokenResponse, Error> {
        let body = serde_json::json!({
            "grantType": "authorization_code",
            "clientId": self.client_id,
            "clientSecret": self.client_secret,
            "code": self.authorization_code,
            "state": 0
        })
        .to_string();

        let response = ChzzkRequestBuilder::open_api("auth/v1/token")
            .post(self, Some(body))
            .send::<Response<AccessTokenResponse>>()
            .await?;

        Ok(response.content)
    }

    /// Get channel live status of channel with given id.
    /// ex) https://api.chzzk.naver.com/polling/v2/channels/1dac6492f81d89e261f692bb6b79ff57/live-status
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    #[cfg(feature = "unofficial")]
    pub async fn live_status(&self, channel_id: &ChannelId) -> Result<ChannelLiveStatus, Error> {
        let response_object = ChzzkRequestBuilder::chzzk(
            format!("polling/v2/channels/{}/live-status", **channel_id).as_str(),
        )
        .get(self, vec![])
        .send::<Response<ChannelLiveStatus>>()
        .await?;

        Ok(response_object.content)
    }

    /// Get user status of currently logged-in user (bot).
    /// ex) https://comm-api.game.naver.com/nng_main/v1/user/getUserStatus
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    #[cfg(feature = "unofficial")]
    pub async fn user_status(&self) -> Result<User, Error> {
        let response_object = ChzzkRequestBuilder::game("v1/user/getUserStatus")
            .get(self, vec![])
            .send::<Response<User>>()
            .await?;
        Ok(response_object.content)
    }

    /// Get AccessToken.
    /// ex) https://comm-api.game.naver.com/nng_main/v1/chats/access-token?channelId=N1SMvY&chatType=STREAMING
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    #[cfg(feature = "unofficial")]
    pub async fn unofficial_access_token(
        &self,
        chat_id: &ChatChannelId,
    ) -> Result<ChatAccessToken, Error> {
        let response_object = ChzzkRequestBuilder::game("v1/chats/access-token")
            .get(
                self,
                vec![
                    ("channelId".to_string(), (**chat_id).clone()),
                    ("chatType".to_string(), "STREAMING".to_string()),
                ],
            )
            .send::<Response<ChatAccessToken>>()
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
    #[cfg(feature = "unofficial")]
    pub async fn channel_info(&self, channel_id: &ChannelId) -> Result<Channel, Error> {
        let response =
            ChzzkRequestBuilder::chzzk(format!("service/v1/channels/{}", **channel_id).as_str())
                .get(self, vec![])
                .send::<Response<Channel>>()
                .await?;

        Ok(response.content)
    }

    #[cfg(feature = "unofficial")]
    pub async fn set_chat_notice(
        &self,
        channel_id: &ChannelId,
        chat_id: Option<&ChatChannelId>,
        sender: &ChannelId,
        time: u64,
    ) -> Result<(), Error> {
        let chat_id = match chat_id {
            Some(x) => x,
            None => &self.live_status(channel_id).await?.chat_channel_id,
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

        ChzzkRequestBuilder::game("v1/chats/notices")
            .post(self, Some(payload.to_string()))
            .send::<DropResponse>()
            .await?;
        Ok(())
    }

    #[cfg(feature = "unofficial")]
    pub async fn search_channels(
        &self,
        keyword: &str,
        offset: usize,
        size: usize,
    ) -> Result<SearchContent<ChannelSearchBundle>, Error> {
        let response = ChzzkRequestBuilder::chzzk("service/v1/search/channels")
            .get(
                self,
                vec![
                    ("keyword".into(), keyword.into()),
                    ("offset".into(), offset.to_string()),
                    ("size".into(), size.to_string()),
                ],
            )
            .send::<Response<SearchContent<ChannelSearchBundle>>>()
            .await?;

        Ok(response.content)
    }

    #[cfg(feature = "unofficial")]
    pub async fn search_videos(
        &self,
        keyword: &str,
        offset: usize,
        size: usize,
    ) -> Result<SearchContent<VideoSearchBundle>, Error> {
        let response = ChzzkRequestBuilder::game("service/v1/search/videos")
            .get(
                self,
                vec![
                    ("keyword".into(), keyword.into()),
                    ("offset".into(), offset.to_string()),
                    ("size".into(), size.to_string()),
                ],
            )
            .send::<Response<SearchContent<VideoSearchBundle>>>()
            .await?;

        Ok(response.content)
    }

    #[cfg(feature = "unofficial")]
    pub async fn search_lives(
        &self,
        keyword: &str,
        offset: usize,
        size: usize,
    ) -> Result<SearchContent<LiveSearchBundle>, Error> {
        let response = ChzzkRequestBuilder::chzzk("service/v1/search/lives")
            .get(
                self,
                vec![
                    ("keyword".into(), keyword.into()),
                    ("offset".into(), offset.to_string()),
                    ("size".into(), size.to_string()),
                ],
            )
            .send::<Response<SearchContent<LiveSearchBundle>>>()
            .await?;

        Ok(response.content)
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
