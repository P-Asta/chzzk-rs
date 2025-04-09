use super::channel::ChannelId;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChannelSearchBundle {
    pub channel: SearchedChannel
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct SearchedChannel {
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub channel_image_url: Option<String>,
    pub verified_mark: bool,
    pub channel_description: String,
    pub follower_count: i32,
    pub open_live: bool,
    pub activated_channel_badge_ids: Option<Vec<String>>,
    // pub personalData: Unknown,
}
