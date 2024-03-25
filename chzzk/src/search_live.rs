#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchLiveJson {
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
    live: Live,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Live {
    liveImageUrl: String,
    defaultThumbnailImageUrl: String,
    concurrentUserCount: usize,
    accumulateCount: usize,
    openDate: String,
    liveId: usize,
    adult: bool,
    chatChannelId: String,
    categoryType: String,
    liveCategory: String,
    liveCategoryValue: String,
    liveTitle: String,
}

impl crate::search::SearchInfo for SearchLiveJson {}
