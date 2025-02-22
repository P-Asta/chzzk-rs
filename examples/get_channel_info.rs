#[tokio::main]
async fn main() {
    let client = chzzk::ChzzkClient::new();
    let res = client
        .live_status(&"00cbd24a8442854ac0b0e2d1ef884776".to_string().into())
        .await;
    println!("{:?}", res.unwrap());
}
