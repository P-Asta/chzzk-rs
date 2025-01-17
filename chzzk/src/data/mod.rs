pub mod channel;
pub mod user;

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

pub(crate) use string_like;
