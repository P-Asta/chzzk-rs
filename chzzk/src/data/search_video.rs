use crate::ChzzkTimestamp;

use super::SimplifiedSearchedChannel;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct VideoSearchBundle {
    pub video: Video,
    pub channel: SimplifiedSearchedChannel,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct Video {
    pub video_no: u32,
    pub video_id: String,
    pub video_title: String,
    pub video_type: String,
    pub publish_date: String,
    pub thumbnail_image_url: String,
    pub duration: i32,
    pub read_count: i32,
    pub channel_id: String,
    pub publish_date_at: ChzzkTimestamp,
    pub adult: bool,
    pub category_type: String,
    pub video_category: String,
    pub video_category_value: String,
    // pub blind_type: Unknown,
    // pub watch_timeline: Unknown,
    pub live_pv: i32,
}
