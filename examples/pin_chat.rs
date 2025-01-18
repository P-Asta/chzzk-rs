use chzzk::ChzzkClient;
mod auth;

#[tokio::main]
async fn main() {
    let mut client = ChzzkClient::new();
    let auth = auth::get_aut_ses_from_env();
    client.sign_in(auth.0.as_str(), auth.1.as_str());
    
    let channel = "1dac6492f81d89e261f692bb6b79ff57".to_string().into();
    client
        .set_chat_notice(
            &channel,
            None,
            &client.get_user_status().await.unwrap().user_id_hash, // these two information
            1737095673004u64,                                      // must match
        )
        .await
        .unwrap();
}
