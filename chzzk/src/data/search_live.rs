use super::{channel::ChannelId, SimplifiedSearchedChannel};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct LiveSearchBundle {
    pub live: SearchedLive,
    pub channel: SimplifiedSearchedChannel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct SearchedLive {
    pub live_title: String,
    pub live_image_url: String,
    pub default_thumbnail_image_url: Option<String>,
    pub concurrent_user_count: u32,
    pub accumulate_count: u32,
    pub open_date: String,
    pub live_id: i32,
    pub adult: bool,
    pub tags: Vec<String>,
    pub chat_channel_id: String,
    pub category_type: String,
    pub live_category: String,
    pub live_category_value: String,
    // pub drops_campaign_no: Unknown,
    // pub watch_party_no: Unknown,
    // pub watch_party_tag: Unknown,
    pub channel_id: ChannelId,
    pub live_playvack_json: String,
    // pub blind_type: Unknown,
}
