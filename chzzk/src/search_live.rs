#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchLiveJson {
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
    pub live: Live,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Live {
    pub liveImageUrl: String,
    pub defaultThumbnailImageUrl: String,
    pub concurrentUserCount: usize,
    pub accumulateCount: usize,
    pub openDate: String,
    pub liveId: usize,
    pub adult: bool,
    pub chatChannelId: String,
    pub categoryType: String,
    pub liveCategory: String,
    pub liveCategoryValue: String,
    pub liveTitle: String,
}

impl crate::search::SearchInfo for SearchLiveJson {}
