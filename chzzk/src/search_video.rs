#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchVideoJson {
    pub code: usize,
    pub message: String,
    pub content: Content,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Content {
    pub size: usize,
    pub data: Vec<Data>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Data {
    pub video: Video,
    pub channel: Channel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Video {
    pub videoId: String,
    pub videoTitle: String,
    pub videoType: String,
    pub publishDate: String,
    pub thumbnailImageUrl: String,
    pub duration: usize,
    pub readCount: usize,
    pub channelId: String,
    pub publishDateAt: usize,
    pub adult: bool,
    pub categoryType: String,
    pub videoCategory: String,
    pub videoCategoryValue: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Channel {
    pub channelId: String,
    pub channelName: String,
    pub channelImageUrl: String,
    pub verifiedMark: bool,
}

// impl crate::search::SearchInfo for SearchChannelJson {}

impl crate::search::SearchInfo for SearchVideoJson {}
