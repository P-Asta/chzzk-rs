use chzzk::{chat::ChatClient, ChzzkClient};

// Find these values in your browser cookies
const AUT: &str = "omitted";
const SES: &str = "omitted";

#[tokio::main]
async fn main() {
    let mut client = ChzzkClient::new();
    client.sign_in(AUT, SES);
    let mut chat_client = ChatClient::new(
        client, "1dac6492f81d89e261f692bb6b79ff57"); // feel free to send chat on this channel
    chat_client.connect().await.unwrap();
    chat_client.send_chat("hello chat").await.unwrap();
}