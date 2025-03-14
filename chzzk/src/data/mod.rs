pub mod channel;
pub mod user;
pub mod search_channel;
pub mod search_live;
pub mod search_video;
macro_rules! string_like {
    ($name: ident) => {
        #[derive(
            serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default,
        )]
        pub struct $name(pub String);

        impl From<String> for $name {
            fn from(v: String) -> Self {
                $name(v)
            }
        }

        impl From<$name> for String {
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl Deref for $name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

use channel::ChannelId;
pub(crate) use string_like;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Response<T> {
    pub code: usize,
    pub message: Option<String>,
    pub content: T,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SearchContent<T> {
    pub size: u32,
    pub data: Vec<T>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[serde(rename_all="camelCase")]
pub struct SimplifiedSearchedChannel {
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub channel_image_url: Option<String>,
    pub verified_mark: bool,
    pub activated_channel_badge_ids: Option<Vec<String>>,
}

impl From<search_channel::SearchedChannel> for SimplifiedSearchedChannel {
    fn from(v: search_channel::SearchedChannel) -> Self {
        SimplifiedSearchedChannel {
            channel_id: v.channel_id,
            channel_name: v.channel_name,
            channel_image_url: v.channel_image_url,
            verified_mark: v.verified_mark,
            activated_channel_badge_ids: v.activated_channel_badge_ids,
        }
    }
}
