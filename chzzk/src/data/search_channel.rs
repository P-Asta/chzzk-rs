use super::channel::ChannelId;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChannelSearchBundle {
    pub channel: SearchedChannel
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchedChannel {
    #[serde(rename = "channelId")]
    pub channel_id: ChannelId,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "channelImageUrl")]
    pub channel_image_url: Option<String>,
    #[serde(rename = "verifiedMark")]
    pub verified_mark: bool,
    #[serde(rename = "channelDescription")]
    pub channel_description: String,
    #[serde(rename = "followerCount")]
    pub follower_count: i32,
    #[serde(rename = "openLive")]
    pub open_live: bool,
    #[serde(rename = "activatedChannelBadgeIds")]
    pub activated_channel_badge_ids: Option<Vec<String>>,
    // pub personalData: Unknown,
}