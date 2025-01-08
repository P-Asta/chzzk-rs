#[tokio::main]
async fn main() {
    let client = chzzk::ChzzkClient::new();
    let res = client.get_channel_live_status(&"00cbd24a8442854ac0b0e2d1ef884776".to_string().into()).await;
    println!("{:?}", res);
}
