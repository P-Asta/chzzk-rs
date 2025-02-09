use crate::ChzzkTimestamp;

use super::SimplifiedSearchedChannel;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct VideoSearchBundle {
    pub video: Video,
    pub channel: SimplifiedSearchedChannel,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Video {
    #[serde(rename = "videoNo")]
    pub video_no: u32,
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "videoTitle")]
    pub video_title: String,
    #[serde(rename = "videoType")]
    pub video_type: String,
    #[serde(rename = "publishDate")]
    pub publish_date: String,
    #[serde(rename = "thumbnailImageUrl")]
    pub thumbnail_image_url: String,
    pub duration: i32,
    #[serde(rename = "readCount")]
    pub read_count: i32,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "publishDateAt")]
    pub publish_date_at: ChzzkTimestamp,
    pub adult: bool,
    #[serde(rename = "categoryType")]
    pub category_type: String,
    #[serde(rename = "videoCategory")]
    pub video_category: String,
    #[serde(rename = "videoCategoryValue")]
    pub video_category_value: String,
    // #[serde(rename = "blindType")]
    // pub blind_type: Unknown,
    // #[serde(rename = "watchTimeline")]
    // pub watch_timeline: Unknown,
    #[serde(rename = "livePv")]
    pub live_pv: i32,
}
