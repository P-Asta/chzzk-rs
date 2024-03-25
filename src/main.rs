use chzzk::{Search, SearchChannelJson, SearchLiveJson, SearchVideoJson};
#[tokio::main]
async fn main() {
    let client = chzzk::Client::new();
    let res = client
        .search::<SearchChannelJson>(Search {
            search_type: chzzk::SearchType::Channel,
            keyword: "마플".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();
    let res = client
        .channel(&res.content.data[0].channel.channelId.clone())
        .await
        .unwrap();
    println!("{:?}", res);
}
