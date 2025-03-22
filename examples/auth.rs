pub struct ExampleAuthentication;

impl ExampleAuthentication {
    pub fn new() -> Self {
        if dotenvy::dotenv().is_err() {
            // success when run in examples directory
            dotenvy::from_path(r#"examples/.env"#).unwrap(); // success when run in chzzk-rs root
        }

        Self
    }

    pub fn aut(&self) -> String {
        std::env::var("CHZZK_AUT").unwrap()
    }

    pub fn ses(&self) -> String {
        std::env::var("CHZZK_SES").unwrap()
    }

    pub fn client_id(&self) -> String {
        std::env::var("CHZZK_CLIENT_ID").unwrap()
    }

    pub fn client_secret(&self) -> String {
        std::env::var("CHZZK_CLIENT_SECRET").unwrap()
    }
}
