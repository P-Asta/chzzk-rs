use std::str::FromStr;

use reqwest::Url;

use crate::{
    authorization::AccessTokenResponse, error::Error, request_builder::ChzzkRequestBuilder,
    Response,
};

#[derive(Clone)]
pub struct ChzzkClient {
    pub(crate) client: reqwest::Client,
    pub client_id: String,
    pub client_secret: String,
    pub authorization_code: Option<String>,
}

#[derive(serde::Deserialize)]
struct DropResponse {}

impl ChzzkClient {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        let c = reqwest::Client::new();

        Self {
            client: c,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            authorization_code: None,
        }
    }

    /// Provides a URL to get authorization code.
    /// User should access this URL with their browser and sign in with their Naver account. After
    /// signing in, user will be redirected to the redirect_uri with authorization code and state.
    /// Check https://chzzk.gitbook.io/chzzk/chzzk-api/authorization for more information.
    pub fn authorization_code_url(&self, redirect_uri: &str, state: &str) -> Url {
        Url::from_str(
            format!(
                "https://chzzk.naver.com/account-interlock?clientId={}&state={}&redirectUri={}",
                self.client_id, redirect_uri, state
            )
            .as_str(),
        )
        .unwrap()
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

    pub async fn revoke_access_token(&self, token: String) -> Result<(), Error> {
        let body = serde_json::json!({
            "clientId": self.client_id,
            "clientSecret": self.client_secret,
            "token": token,
            "tokenTypeHint": "access_token"
        })
        .to_string();

        ChzzkRequestBuilder::open_api("auth/v1/token/revoke")
            .post(self, Some(body))
            .send::<DropResponse>()
            .await?;
        Ok(())
    }

    pub async fn revoke_refresh_token(&self, token: String) -> Result<(), Error> {
        let body = serde_json::json!({
            "clientId": self.client_id,
            "clientSecret": self.client_secret,
            "token": token,
            "tokenTypeHint": "refresh_token"
        })
        .to_string();

        ChzzkRequestBuilder::open_api("auth/v1/token/revoke")
            .post(self, Some(body))
            .send::<DropResponse>()
            .await?;
        Ok(())
    }
}
