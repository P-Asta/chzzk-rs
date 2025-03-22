use reqwest::RequestBuilder;

use crate::{
    debug_println,
    error::{chain_error, Error},
    ChzzkClient,
};

pub(crate) struct ChzzkRequestBuilder {
    url: String,
}

impl ChzzkRequestBuilder {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn from_path(path: &str) -> Self {
        ChzzkRequestBuilder::new(format!("https://openapi.chzzk.naver.com/{path}"))
    }

    pub fn get(self, client: &ChzzkClient, param: Vec<(String, String)>) -> ChzzkRequestWrapper {
        let url = self.url
            + "?"
            + &param
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&");
        debug_println!("request to: {}.", url);
        let request = client
            .client
            .get(url)
            .header("Client-id", &client.client_id)
            .header("Client-secret", &client.client_secret)
            .header("Content-type", "application/json");

        ChzzkRequestWrapper { request }
    }

    pub fn post(self, client: &ChzzkClient, body: Option<String>) -> ChzzkRequestWrapper {
        debug_println!("request to: {}.", self.url);

        let request = client
            .client
            .post(self.url)
            .header("Client-id", &client.client_id)
            .header("Client-secret", &client.client_secret)
            .header("Content-type", "application/json")
            .body(body.unwrap_or_default());

        ChzzkRequestWrapper { request }
    }
}

pub(super) struct ChzzkRequestWrapper {
    request: RequestBuilder,
}

impl ChzzkRequestWrapper {
    pub async fn send<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let response = self
            .request
            .send()
            .await
            .map_err(chain_error("do_request: failed to get response"))?;

        let text = response
            .text()
            .await
            .map_err(chain_error("do_request: response is not a text"))?;

        let json = serde_json::from_str::<T>(&text)
            // let json = json::parse(text.as_str())
            .map_err(chain_error(
                format!("do_request: response is not a json. {}", text).as_str(),
            ))?;

        Ok(json)
    }
}
