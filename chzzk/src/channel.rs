#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Channel {
    pub code: usize,
    pub message: String,
    pub content: Content,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct Content {
    pub channelId: String,
    pub channelName: String,
    pub channelImageUrl: String,
    pub verifiedMark: bool,
    pub channelType: String,
    pub channelDescription: String,
    pub followerCount: usize,
    pub openLive: bool,
    pub subscriptionAvailability: bool,
    pub subscriptionPaymentAvailability: SubscriptionPaymentAvailability,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
pub struct SubscriptionPaymentAvailability {
    pub iapAvailability: bool,
    pub iabAvailability: bool,
}
