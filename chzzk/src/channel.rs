#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChannelResponse {
    pub code: usize,
    pub message: String,
    pub content: Channel,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Channel {
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "channelImageUrl")]
    pub channel_image_url: String,
    #[serde(rename = "verifiedMark")]
    pub verified_mark: bool,
    #[serde(rename = "channelType")]
    pub channel_type: String,
    #[serde(rename = "channelDescription")]
    pub channel_description: String,
    #[serde(rename = "followerCount")]
    pub follower_count: usize,
    #[serde(rename = "openLive")]
    pub open_live: bool,
    #[serde(rename = "subscriptionAvailability")]
    pub subscription_availability: bool,
    #[serde(rename = "subscriptionPaymentAvailability")]
    pub subscription_payment_availability: SubscriptionPaymentAvailability,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SubscriptionPaymentAvailability {
    #[serde(rename = "iapAvailability")]
    pub iap_availability: bool,
    #[serde(rename = "iabAvailability")]
    pub iab_availability: bool,
}
