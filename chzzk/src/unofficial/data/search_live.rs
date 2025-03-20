use super::SimplifiedSearchedChannel;
use crate::channel::ChannelId;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct LiveSearchBundle {
    pub live: SearchedLive,
    pub channel: SimplifiedSearchedChannel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchedLive {
    #[serde(rename = "liveTitle")]
    pub live_title: String,
    #[serde(rename = "liveImageUrl")]
    pub live_image_url: String,
    #[serde(rename = "defaultThumbnailImageUrl")]
    pub default_thumbnail_image_url: Option<String>,
    #[serde(rename = "concurrentUserCount")]
    pub concurrent_user_count: u32,
    #[serde(rename = "accumulateCount")]
    pub accumulate_count: u32,
    #[serde(rename = "openDate")]
    pub open_date: String,
    #[serde(rename = "liveId")]
    pub live_id: i32,
    pub adult: bool,
    pub tags: Vec<String>,
    #[serde(rename = "chatChannelId")]
    pub chat_channel_id: String,
    #[serde(rename = "categoryType")]
    pub category_type: String,
    #[serde(rename = "liveCategory")]
    pub live_category: String,
    #[serde(rename = "liveCategoryValue")]
    pub live_category_value: String,
    // #[serde(rename = "dropsCampaignNo")]
    // pub drops_campaign_no: Unknown,
    // #[serde(rename = "watchPartyNo")]
    // pub watch_party_no: Unknown,
    // #[serde(rename = "watchPartyTag")]
    // pub watch_party_tag: Unknown,
    #[serde(rename = "channelId")]
    pub channel_id: ChannelId,
    #[serde(rename = "livePlayvackJson")]
    pub live_playvack_json: String,
    // #[serde(rename = "blindType")]
    // pub blind_type: Unknown,
}
