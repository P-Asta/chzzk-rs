#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchChannelJson {
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
    channel: Channel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Channel {
    channelId: String,
    channelName: String,
    channelImageUrl: String,
    verifiedMark: bool,
    channelDescription: String,
    followerCount: usize,
    openLive: bool,
}

impl crate::search::SearchInfo for SearchChannelJson {}
