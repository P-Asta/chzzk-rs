use reqwest::RequestBuilder;

use crate::{
    debug_println,
    error::{chain_error, Error},
    ChzzkClient,
};

#[cfg(feature = "unofficial")]
#[derive(Clone)]
pub(super) struct Nid {
    pub aut: String,
    pub ses: String,
}

pub(crate) struct ChzzkRequestBuilder {
    url: String,
}

impl ChzzkRequestBuilder {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    #[cfg(feature = "unofficial")]
    pub fn chzzk(path: &str) -> Self {
        ChzzkRequestBuilder::new(format!("https://api.chzzk.naver.com/{path}"))
    }

    #[cfg(feature = "unofficial")]
    pub fn game(path: &str) -> Self {
        ChzzkRequestBuilder::new(format!("https://comm-api.game.naver.com/nng_main/{path}"))
    }

    pub fn open_api(path: &str) -> Self {
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
        let request = client.client.get(url);

        #[cfg(feature = "unofficial")]
        return ChzzkRequestWrapper {
            request,
            nid: client.nid.clone(),
        };

        #[cfg(not(feature = "unofficial"))]
        ChzzkRequestWrapper { request }
    }

    pub fn post(self, client: &ChzzkClient, body: Option<String>) -> ChzzkRequestWrapper {
        debug_println!("request to: {}.", self.url);

        let request = client
            .client
            .post(self.url)
            .header("Content-Type", "application/json")
            .body(body.unwrap_or_default());

        #[cfg(feature = "unofficial")]
        return ChzzkRequestWrapper {
            request,
            nid: client.nid.clone(),
        };

        #[cfg(not(feature = "unofficial"))]
        ChzzkRequestWrapper { request }
    }
}

pub(super) struct ChzzkRequestWrapper {
    request: RequestBuilder,
    #[cfg(feature = "unofficial")]
    nid: Option<Nid>,
}

impl ChzzkRequestWrapper {
    pub async fn send<T: serde::de::DeserializeOwned>(mut self) -> Result<T, Error> {
        #[cfg(feature = "unofficial")]
        if let Some(nid) = self.nid {
            self.request = self
                .request
                .header("Cookie", format!("NID_AUT={};NID_SES={}", nid.aut, nid.ses));
        }

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
