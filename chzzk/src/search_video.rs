#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchVideoJson {
    code: usize,
    message: String,
    content: Content,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Content {
    size: usize,
    data: Vec<Data>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Data {
    video: Video,
    channel: Channel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Video {
    videoId: String,
    videoTitle: String,
    videoType: String,
    publishDate: String,
    thumbnailImageUrl: String,
    duration: usize,
    readCount: usize,
    channelId: String,
    publishDateAt: usize,
    adult: bool,
    categoryType: String,
    videoCategory: String,
    videoCategoryValue: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Channel {
    channelId: String,
    channelName: String,
    channelImageUrl: String,
    verifiedMark: bool,
}

// impl crate::search::SearchInfo for SearchChannelJson {}

impl crate::search::SearchInfo for SearchVideoJson {}
