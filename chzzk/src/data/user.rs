use serde::{Deserialize, Deserializer};

use super::channel::ChannelId;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub has_profile: bool,
    pub user_id_hash: ChannelId,
    pub nickname: String,
    pub profile_image_url: String,
    // pub penalties: Unknown,
    pub official_noti_agree: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub official_noti_agree_updated_date: String,
    pub verified_mark: bool,
    pub logged_in: bool,
}

fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let key = Option::<T>::deserialize(de)?;
    Ok(key.unwrap_or_default())
}
