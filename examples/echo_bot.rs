use std::time::Duration;
use chzzk::{chat::ChatClient, ChzzkClient};
mod auth;

#[tokio::main]
async fn main() {
    let auth = auth::get_aut_ses_from_env();
    let client = ChzzkClient::new_with_sign_in(auth.0.as_str(), auth.1.as_str());
    let mut chat = ChatClient::new(client, "1dac6492f81d89e261f692bb6b79ff57");
    chat.connect().await.unwrap();
    let cc = chat.clone();
    chat.register_on_chat(|x| async move {
        println!("{}", x.message);
        cc.send_chat(x.message.as_str()).await.unwrap();
    })
    .await;
    tokio::time::sleep(Duration::from_secs(120)).await;
}