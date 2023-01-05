use std::env;
use log::LevelFilter;
use serde_json::json;
use crypto_botters::{Client, bybit::{BybitOption}};
use crypto_botters_bybit::BybitHttpAuth;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BYBIT_API_KEY").expect("no API key found");
    let secret = env::var("BYBIT_API_SECRET").expect("no API secret found");
    let mut client = Client::new();
    client.default_option(BybitOption::Key(key));
    client.default_option(BybitOption::Secret(secret));

    // private POST
    let batch_cancel: serde_json::Value = client.post(
        "/spot/v3/private/cancel-orders",
        Some(json!({"symbol": "BTCUSDT"})),
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to cancel orders");
    println!("Batch cancel result:\n{}", batch_cancel);

    // private GET
    let open_orders: serde_json::Value = client.get_no_query(
        "/spot/v3/private/open-orders",
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to get orders");
    println!("Open orders:\n{}", open_orders);

    // public GET
    let last_price: serde_json::Value = client.get(
        "/spot/v3/public/quote/ticker/price",
        Some(&[("symbol", "BTCUSDT")]),
        [BybitOption::Default],
    ).await.expect("failed to get price");
    println!("Last price:\n{}", last_price);
}
