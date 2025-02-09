use crate::{
    channel::{Channel, ChannelId, ChannelLiveStatus, ChatAccessToken, ChatChannelId}, error::Error, request_builder::{ChzzkRequestBuilder, Nid}, search_channel::ChannelSearchBundle, search_live::LiveSearchBundle, search_video::VideoSearchBundle, user::User, Response, SearchContent
};

#[derive(Clone)]
pub struct ChzzkClient {
    pub(super) nid: Option<Nid>,
    pub(super) client: reqwest::Client,
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
    pub async fn get_user_status(&self) -> Result<User, Error> {
        let response_object = Self::game("v1/user/getUserStatus")
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
        .get(self, vec![])
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
    pub async fn get_channel_info(&self, channel_id: &ChannelId) -> Result<Channel, Error> {
        let response = Self::chzzk(format!("service/v1/channels/{}", **channel_id).as_str())
            .get(self, vec![])
            .send::<Response<Channel>>()
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
            .post(self, Some(payload.to_string()))
            .send::<DropResponse>()
            .await?;
        Ok(())
    }

    pub async fn search_channels(
        &self,
        keyword: &str,
        offset: usize,
        size: usize,
    ) -> Result<SearchContent<ChannelSearchBundle>, Error> {
        let response = Self::chzzk("service/v1/search/channels")
            .get(self, vec![("keyword".into(), keyword.into()), ("offset".into(), offset.to_string()), ("size".into(), size.to_string())])
            .send::<Response<SearchContent<ChannelSearchBundle>>>()
            .await?;

        Ok(response.content)
    }

    pub async fn search_videos(
        &self,
        keyword: &str,
        offset: usize,
        size: usize,
    ) -> Result<SearchContent<VideoSearchBundle>, Error> {
        let response = Self::game("service/v1/search/videos")
            .get(self, vec![("keyword".into(), keyword.into()), ("offset".into(), offset.to_string()), ("size".into(), size.to_string())])
            .send::<Response<SearchContent<VideoSearchBundle>>>()
            .await?;

        Ok(response.content)
    }

    pub async fn search_lives(
        &self,
        keyword: &str,
        offset: usize,
        size: usize,
    ) -> Result<SearchContent<LiveSearchBundle>, Error> {
        let response = Self::chzzk("service/v1/search/lives")
            .get(self, vec![("keyword".into(), keyword.into()), ("offset".into(), offset.to_string()), ("size".into(), size.to_string())])
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
