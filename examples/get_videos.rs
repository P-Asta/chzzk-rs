use chzzk::{Search, SearchLiveJson};
#[tokio::main]
async fn main() {
    let client = chzzk::Client::new();
    let res = client
        .search::<SearchLiveJson>(Search {
            search_type: chzzk::SearchType::Video,
            keyword: "공포".to_string(),
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
}
