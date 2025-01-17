/// You need to fill `.env` with your own AUT and SES values from browser cookies
/// in order to run some examples.
pub fn get_aut_ses_from_env() -> (String, String) {
    if dotenvy::dotenv().is_err() {
        // success when run in examples directory
        dotenvy::from_path(r#"examples/.env"#).unwrap(); // success when run in chzzk-rs root
    }
    (
        std::env::var("CHZZK_AUT").unwrap(),
        std::env::var("CHZZK_SES").unwrap(),
    )
}
