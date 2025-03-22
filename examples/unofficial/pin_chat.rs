use chzzk::unofficial::ChzzkClient;
#[path = "../auth.rs"]
mod auth;

#[tokio::main]
async fn main() {
    let mut client = ChzzkClient::new();
    let auth = auth::ExampleAuthentication::new();
    client.sign_in(&auth.aut(), &auth.ses());

    let channel = "1dac6492f81d89e261f692bb6b79ff57".to_string().into();
    client
        .set_chat_notice(
            &channel,
            None,
            &client.user_status().await.unwrap().user_id_hash, // these two information
            1737095673004u64,                                  // must match
        )
        .await
        .unwrap();
}
