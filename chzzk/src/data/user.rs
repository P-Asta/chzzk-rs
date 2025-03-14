use super::channel::ChannelId;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct User {
    pub has_profile: bool,
    pub user_id_hash: ChannelId,
    pub nickname: String,
    pub profile_image_url: String,
    // pub penalties: Unknown,
    pub official_noti_agree: bool,
    pub official_noti_agree_updated_date: String,
    pub verified_mark: bool,
    pub logged_in: bool,
}
