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

        impl Into<$name> for &str {
            fn into(self) -> $name {
                self.to_string().into()
            }
        }

        impl Deref for $name {
            type Target = String;
        
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name { 
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

pub(crate) use string_like;
