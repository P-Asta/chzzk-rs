use super::string_like;
use crate::ChzzkDateTime;
use std::ops::Deref;

string_like! {ChannelId} // Hexadecimal channel id
string_like! {ChatChannelId} // 6-letters-long chat id

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct Channel {
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub channel_image_url: Option<String>,
    pub verified_mark: bool,
    pub channel_type: String,
    pub channel_description: String,
    pub follower_count: usize,
    pub open_live: bool,
    pub subscription_availability: bool,
    pub subscription_payment_availability: SubscriptionPaymentAvailability,
    pub ad_monetization_availability: bool,
    // pub activated_channel_badge_ids: Unknown,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct SubscriptionPaymentAvailability {
    pub iap_availability: bool,
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
#[serde(rename_all="camelCase")]
pub struct ChannelLiveStatus {
    pub live_title: String,
    pub status: LiveStatus,
    pub concurrent_user_count: i32,
    pub accumulate_count: i32,
    pub paid_promotion: bool,
    pub adult: bool,
    pub kr_only_viewing: bool,
    pub open_date: Option<ChzzkDateTime>,
    pub close_date: Option<ChzzkDateTime>,
    pub clip_active: bool,
    pub chat_channel_id: ChatChannelId,
    pub tags: Vec<String>,
    pub category_type: Option<String>,
    pub live_category: Option<String>,
    pub live_category_value: String,
    // pub fault_status: Unknown,
    pub user_adult_status: String,
    // pub blind_type: Unknown,
    // pub player_recommend_content: Unknown,
    pub chat_active: bool,
    pub chat_available_group: String,
    pub chat_available_condition: String,
    pub min_follower_minute: i32,
    pub allow_subscriber_in_follower_mode: bool,
    pub chat_donation_ranking_exposure: bool,
    // pub drops_campaign_no: Unknown,
    // pub live_token_list: Unknown,
    // pub watch_party_no: Unknown,
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
#[serde(rename_all="camelCase")]
pub struct ChatAccessToken {
    pub access_token: String,
    // pub temporary_restrict: Unknown,
    pub real_name_auth: bool,
    pub extra_token: String,
}
