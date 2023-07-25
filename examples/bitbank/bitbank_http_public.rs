use log::LevelFilter;
use crypto_botters::{
    Client,
    bitbank::BitbankOption,
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    let transactions: serde_json::Value = client.get_no_query(
        "/btc_jpy/transactions",
        [BitbankOption::Default],
    ).await.expect("failed to get transactions");
    println!("BTC transactions:\n{:?}", transactions);
}
