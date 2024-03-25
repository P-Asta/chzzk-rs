use chzzk::{Search, SearchChannelJson};
#[tokio::main]
async fn main() {
    let client = chzzk::Client::new();
    let res = client
        .search::<SearchChannelJson>(Search {
            search_type: chzzk::SearchType::Channel,
            keyword: "공포".to_string(),
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
}
