use log::LevelFilter;
use crypto_botters::{Client, bybit::{BybitOption}};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    // public GET
    let symbols: serde_json::Value = client.get_no_query(
        "/contract/v3/public/copytrading/symbol/list",
        [BybitOption::Default],
    ).await.expect("failed to symbol list");
    println!("Symbols:\n{}", symbols);
}
