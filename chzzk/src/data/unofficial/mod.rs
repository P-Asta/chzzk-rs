use crate::channel::ChannelId;

pub mod channel;
pub mod search_channel;
pub mod search_live;
pub mod search_video;
pub mod user;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchContent<T> {
    pub size: u32,
    pub data: Vec<T>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SimplifiedSearchedChannel {
    #[serde(rename = "channelId")]
    pub channel_id: ChannelId,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "channelImageUrl")]
    pub channel_image_url: Option<String>,
    #[serde(rename = "verifiedMark")]
    pub verified_mark: bool,
    #[serde(rename = "activatedChannelBadgeIds")]
    pub activated_channel_badge_ids: Option<Vec<String>>,
}

impl From<search_channel::SearchedChannel> for SimplifiedSearchedChannel {
    fn from(v: search_channel::SearchedChannel) -> Self {
        SimplifiedSearchedChannel {
            channel_id: v.channel_id,
            channel_name: v.channel_name,
            channel_image_url: v.channel_image_url,
            verified_mark: v.verified_mark,
            activated_channel_badge_ids: v.activated_channel_badge_ids,
        }
    }
}
