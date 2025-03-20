#[tokio::main]
async fn main() {
    let client = chzzk::unofficial::ChzzkClient::new();
    let res = client.search_channels("탬탬", 0, 20).await.unwrap();

    for (index, channel) in res.data.iter().enumerate() {
        println!("{}: {}", index + 1, channel.channel.channel_name);
    }
}
