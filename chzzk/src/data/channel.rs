use super::string_like;
use crate::ChzzkDateTime;
use std::ops::Deref;

string_like! {ChannelId} // Hexadecimal channel id
string_like! {ChatChannelId} // 6-letters-long chat id

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Channel {
    #[serde(rename = "channelId")]
    pub channel_id: ChannelId,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "channelImageUrl")]
    pub channel_image_url: Option<String>,
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
    #[serde(rename = "adMonetizationAvailability")]
    pub ad_monetization_availability: bool,
    // #[serde(rename = "activatedChannelBadgeIds")]
    // pub activated_channel_badge_ids: Unknown,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SubscriptionPaymentAvailability {
    #[serde(rename = "iapAvailability")]
    pub iap_availability: bool,
    #[serde(rename = "iabAvailability")]
    pub iab_availability: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum LiveStatus {
    #[default]
    Close,
    Open,
}

impl<'de> serde::Deserialize<'de> for LiveStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "CLOSE" => Ok(LiveStatus::Close),
            "OPEN" => Ok(LiveStatus::Open),
            _ => Err(serde::de::Error::custom("unexpected value")),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChannelLiveStatus {
    #[serde(rename = "liveTitle")]
    pub live_title: String,
    pub status: LiveStatus,
    #[serde(rename = "concurrentUserCount")]
    pub concurrent_user_count: i32,
    #[serde(rename = "accumulateCount")]
    pub accumulate_count: i32,
    #[serde(rename = "paidPromotion")]
    pub paid_promotion: bool,
    pub adult: bool,
    #[serde(rename = "krOnlyViewing")]
    pub kr_only_viewing: bool,
    #[serde(rename = "openDate")]
    pub open_date: Option<ChzzkDateTime>,
    #[serde(rename = "closeDate")]
    pub close_date: Option<ChzzkDateTime>,
    #[serde(rename = "clipActive")]
    pub clip_active: bool,
    #[serde(rename = "chatChannelId")]
    pub chat_channel_id: ChatChannelId,
    pub tags: Vec<String>,
    #[serde(rename = "categoryType")]
    pub category_type: Option<String>,
    #[serde(rename = "liveCategory")]
    pub live_category: Option<String>,
    #[serde(rename = "liveCategoryValue")]
    pub live_category_value: String,
    // #[serde(rename = "faultStatus")]
    // pub fault_status: Unknown,
    #[serde(rename = "userAdultStatus")]
    pub user_adult_status: String,
    // #[serde(rename = "blindType")]
    // pub blind_type: Unknown,
    // #[serde(rename = "playerRecommendContent")]
    // pub player_recommend_content: Unknown,
    #[serde(rename = "chatActive")]
    pub chat_active: bool,
    #[serde(rename = "chatAvailableGroup")]
    pub chat_available_group: String,
    #[serde(rename = "chatAvailableCondition")]
    pub chat_available_condition: String,
    #[serde(rename = "minFollowerMinute")]
    pub min_follower_minute: i32,
    #[serde(rename = "allowSubscriberInFollowerMode")]
    pub allow_subscriber_in_follower_mode: bool,
    #[serde(rename = "chatDonationRankingExposure")]
    pub chat_donation_ranking_exposure: bool,
    // #[serde(rename = "dropsCampaignNo")]
    // pub drops_campaign_no: Unknown,
    // #[serde(rename = "liveTokenList")]
    // pub live_token_list: Unknown,
    // #[serde(rename = "watchPartyNo")]
    // pub watch_party_no: Unknown,
    // #[serde(rename = "watchPartyTag")]
    // pub watch_party_tag: Unknown,
}

impl ChannelLiveStatus {
    pub fn open_or<E>(self, err: E) -> Result<Self, E> {
        if self.status == LiveStatus::Open {
            Ok(self)
        } else {
            Err(err)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChatAccessToken {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    // #[serde(rename = "temporaryRestrict")]
    // pub temporary_restrict: Unknown,
    #[serde(rename = "realNameAuth")]
    pub real_name_auth: bool,
    #[serde(rename = "extraToken")]
    pub extra_token: String,
}
