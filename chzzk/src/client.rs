use json::{object::Object, JsonValue};

use crate::r#macro::unwrap_content;

use super::{chzzk_error::{chain_error, Error}, 
r#macro::{jsonvalue_unwrap_or_return, simple_get, simple_get_as}};

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

pub struct LiveChannelStatus {
    pub chat_id: String,
    pub current_user_count: i32,
    pub total_user_count: i32,
    pub title: String,
    pub status: String,
}

pub struct ChatStatus {
    pub access_token: String,
    pub extra_token: String,
}

#[derive(Clone)]
pub struct User(pub String);

// const { chzzkBaseURL, gameBaseURL, naverBaseURL, NID } = require("./val.js");
// const vm = require("./vm.js");

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
    async fn do_request(&self, full_path: String) -> Result<Object, Error> {
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

        let json = json::parse(text.as_str())
            .map_err(chain_error!("do_request: response is not a json"))?;

        Ok(jsonvalue_unwrap_or_return!(
            JsonValue::Object,
            json
        )?)
    }

    async fn request_chzzk(&self, path: String) -> Result<Object, Error> {
        self.do_request(format!("https://api.chzzk.naver.com/{path}"))
            .await
    }

    async fn request_game(&self, path: String) -> Result<Object, Error> {
        self.do_request(format!("https://comm-api.game.naver.com/nng_main/{path}"))
            .await
    }

    pub async fn get_live_channel_status(
        &self,
        channel_id: &str,
    ) -> Result<Option<LiveChannelStatus>, Error> {
        let response_object = self
            .request_chzzk(format!("polling/v2/channels/{channel_id}/live-status"))
            .await?;

        let content = simple_get!(response_object, "content")?;
        let content_object = match content {
            JsonValue::Null => return Ok(None), // currently not livestreaming
            JsonValue::Object(object) => object,
            _ => todo!(),
        };

        // Get further information from content_object
        let chat_id = simple_get_as!(content_object, "chatChannelId", as_str)?
            .to_string();
        let current_user_count = simple_get_as!(
            content_object,
            "concurrentUserCount",
            as_i32
        )?;
        let total_user_count = simple_get_as!(
            content_object,
            "accumulateCount",
            as_i32
        )?;
        let title =
            simple_get_as!(content_object, "liveTitle", as_str)?.to_string();
        let status =
            simple_get_as!(content_object, "status", as_str)?.to_string();
        // let polling = simple_get!(content_object, .parse(res.livePollingStatusJson)

        Ok(Some(LiveChannelStatus {
            chat_id,
            current_user_count,
            total_user_count,
            title,
            status,
        }))
    }

    /// Get user status of currently logged-in user (bot).
    /// ex) https://comm-api.game.naver.com/nng_main/v1/user/getUserStatus
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_user_status(&self) -> Result<User, Error> {
        let response_object = self.request_game("v1/user/getUserStatus".into()).await?;

        let content_object = unwrap_content!(response_object);
        Ok(User(
            simple_get_as!(content_object, "userIdHash", as_str)?.to_string(),
        ))
    }

    /// Get AccessToken.
    /// ex) https://comm-api.game.naver.com/nng_main/v1/chats/access-token?channelId=N1SMvY&chatType=STREAMING
    ///
    /// # Errors
    ///
    /// This function will return an error if request fails.
    pub async fn get_access_token(&self, chat_id: &str) -> Result<ChatStatus, Error> {
        let response_object = self
            .request_game(format!(
                "v1/chats/access-token?channelId={chat_id}&chatType=STREAMING"
            ))
            .await?;

        let content_object = unwrap_content!(response_object);

        // let code = simple_get_as!(content_object, "code", as_i32, get_access_token)?;

        // if code == 42601 {
        //     return Err("Broadcast is age-restricted, nidAuth, nidSession is required".into())
        // }

        let access_token =
            simple_get_as!(content_object, "accessToken", as_str)?.to_string();
        let extra_token =
            simple_get_as!(content_object, "extraToken", as_str)?.to_string();
        Ok(ChatStatus {
            access_token,
            extra_token,
        })
    }
}

impl Default for ChzzkClient {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::test]
pub async fn test_live() {
    use crate::env::{AUT, SES};
    let b = ChzzkClient::new_with_sign_in(AUT, SES); // get it from your cookie!
    let channel_id = "1dac6492f81d89e261f692bb6b79ff57";
    b.get_live_channel_status(channel_id)
        .await
        .unwrap()
        .unwrap();
}
