mod channel;
mod chat;
mod search;
mod search_channel;
mod search_live;
mod search_video;
use channel::Channel;
pub use search_channel::SearchChannelJson;
pub use search_live::SearchLiveJson;
pub use search_video::SearchVideoJson;

pub use search::{Search, SearchInfo, SearchType};
pub struct Client {
    client: reqwest::Client,
}
impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }
    /// # Live
    /// ```rs
    /// let client = chzzk::Client::new();
    /// let res = client
    ///     .search::<SearchLiveJson>(Search {
    ///         search_type: chzzk::SearchType::Live,
    ///         keyword: "공포".to_string(),
    ///         ..Default::default()
    ///     })
    ///     .await;
    /// println!("{:?}", res);
    /// ```
    /// # Channel
    /// ```rs
    /// let client = chzzk::Client::new();
    /// let res = client
    ///     .search::<SearchChannelJson>(Search {
    ///         search_type: chzzk::SearchType::Channel,
    ///         keyword: "공포".to_string(),
    ///         ..Default::default()
    ///     })
    ///     .await;
    /// println!("{:?}", res);
    /// ```
    /// # Video
    /// ```rs
    /// let client = chzzk::Client::new();
    /// let res = client
    ///     .search::<SearchVideoJson>(Search {
    ///         search_type: chzzk::SearchType::Video,
    ///         keyword: "공포".to_string(),
    ///         ..Default::default()
    ///     })
    ///     .await;
    /// println!("{:?}", res);
    /// ```

    pub async fn search<T: SearchInfo + serde::de::DeserializeOwned>(
        &self,
        search: search::Search,
    ) -> Result<T, String> {
        let url = format!(
            "{}?keyword={:?}&offset={:?}&size={:?}",
            match search.search_type {
                search::SearchType::Channel =>
                    "https://api.chzzk.naver.com/service/v1/search/channels",
                search::SearchType::Video => "https://api.chzzk.naver.com/service/v1/search/videos",
                search::SearchType::Live => "https://api.chzzk.naver.com/service/v1/search/lives",
            },
            search.keyword,
            search.offset,
            search.size
        );
        let res = self.client.get(url).send().await;

        if res.is_ok() {
            let res =
                serde_json::from_str(&res.unwrap().text().await.unwrap().replace("null", "\"\""));
            if res.is_ok() {
                return Ok(res.unwrap());
            }
            return Err("json parsing failed".to_string());
        }
        Err(format!("{:?}", res.err()))
    }

    pub async fn channel(&self, channel_id: &str) -> Result<Channel, String> {
        let url = format!(
            "https://api.chzzk.naver.com/service/v1/channels/{}",
            channel_id
        );
        let res = self.client.get(url).send().await;

        if res.is_ok() {
            let res =
                serde_json::from_str(&res.unwrap().text().await.unwrap().replace("null", "\"\""));
            if res.is_ok() {
                return Ok(res.unwrap());
            }
            return Err("json parsing failed".to_string());
        }
        Err(format!("{:?}", res.err()))
    }
}
