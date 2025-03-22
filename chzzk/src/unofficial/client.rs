use crate::{
    channel::ChannelId,
    error::Error,
    unofficial::{
        channel::{Channel, ChannelLiveStatus, ChatAccessToken, ChatChannelId},
        request_builder::{ChzzkRequestBuilder, Nid},
        search_channel::ChannelSearchBundle,
        search_live::LiveSearchBundle,
        search_video::VideoSearchBundle,
        user::User,
        SearchContent,
    },
    Response,
};

#[derive(Clone)]
pub struct ChzzkClient {
    pub(crate) client: reqwest::Client,
    pub(super) nid: Option<Nid>,
}

#[derive(serde::Deserialize)]
struct DropResponse {}

impl ChzzkClient {
    pub fn new() -> Self {
        let c = reqwest::Client::new();

        return Self {
            client: c,
            nid: None,
        };
    }

    pub fn sign_in(&mut self, aut: &str, ses: &str) {
        let nid = Nid {
            aut: aut.into(),
            ses: ses.into(),
        };
        self.nid = Some(nid);
    }

    /// Get channel live status of channel with given id.
    /// ex) https://api.chzzk.naver.com/polling/v2/channels/1dac6492f81d89e261f692bb6b79ff57/live-status
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
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
