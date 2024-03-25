#[tokio::main]
async fn main() {
    let client = chzzk::Client::new();
    let res = client.channel("00cbd24a8442854ac0b0e2d1ef884776").await;
    println!("{:?}", res);
}
