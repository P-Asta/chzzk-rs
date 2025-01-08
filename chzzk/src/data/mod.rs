pub mod channel;
pub mod user;


macro_rules! string_like {
    ($name: ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
        pub struct $name(pub String);

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0
            }
        }

        impl Into<$name> for String {
            fn into(self) -> $name {
                $name(self)
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
