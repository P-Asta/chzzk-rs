use chzzk::unofficial::{chat::ChatClient, ChzzkClient};
#[path = "../auth.rs"]
mod auth;

#[tokio::main]
async fn main() {
    let auth = auth::ExampleAuthentication::new();
    let mut client = ChzzkClient::new();
    client.sign_in(&auth.aut(), &auth.ses());
    let mut chat_client = ChatClient::new(
        client,
        &"1dac6492f81d89e261f692bb6b79ff57".to_string().into(),
    ); // feel free to send chat on this channel
    chat_client.connect().await.unwrap();
    chat_client.send_chat("hello chat").await.unwrap();
}
