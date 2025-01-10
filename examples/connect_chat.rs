use chzzk::{chat::ChatClient, ChzzkClient};
mod auth;

#[tokio::main]
async fn main() {
    let mut client = ChzzkClient::new();
    let auth = auth::get_aut_ses_from_env();
    client.sign_in(auth.0.as_str(), auth.1.as_str());
    let mut chat_client = ChatClient::new(
        client,
        &"1dac6492f81d89e261f692bb6b79ff57".to_string().into(),
    ); // feel free to send chat on this channel
    chat_client.connect().await.unwrap();
    chat_client.send_chat("hello chat").await.unwrap();
}
