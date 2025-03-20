use chzzk::unofficial::{chat::ChatClient, ChzzkClient};
use std::time::Duration;
#[path ="../auth.rs"]
mod auth;

#[tokio::main]
async fn main() {
    let auth = auth::ExampleAuthentication::new();

    let mut client = ChzzkClient::new();
    client.sign_in(&auth.aut(), &auth.ses());

    let mut chat = ChatClient::new(
        client,
        &"1dac6492f81d89e261f692bb6b79ff57".to_string().into(),
    );
    chat.connect().await.unwrap();

    let cc = chat.clone();
    chat.register_on_chat(|x| async move {
        println!("{}", x.message);
        cc.send_chat(x.message.as_str()).await.unwrap();
    })
    .await;

    tokio::time::sleep(Duration::from_secs(120)).await;
}
