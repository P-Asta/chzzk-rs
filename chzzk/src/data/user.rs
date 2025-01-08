use std::ops::Deref;
use super::string_like;

string_like! {UserIdHash}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct User {
    #[serde(rename = "hasProfile")]
    pub has_profile: bool,
    #[serde(rename = "userIdHash")]
    pub user_id_hash: UserIdHash,
    pub nickname: String,
    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: String,
    // pub penalties: Unknown,
    #[serde(rename = "officialNotiAgree")]
    pub official_noti_agree: bool,
    #[serde(rename = "officialNotiAgreeUpdatedDate")]
    pub official_noti_agree_updated_date: String,
    #[serde(rename = "verifiedMark")]
    pub verified_mark: bool,
    #[serde(rename = "loggedIn")]
    pub logged_in: bool,
}
