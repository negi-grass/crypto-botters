use std::env;
use log::LevelFilter;
use serde_json::json;
use crypto_botters::{Client, bybit::{BybitOption, BybitHttpAuth}};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BYBIT_API_KEY").expect("no API key found");
    let secret = env::var("BYBIT_API_SECRET").expect("no API secret found");
    let mut client = Client::new();
    client.update_default_option(BybitOption::Key(key));
    client.update_default_option(BybitOption::Secret(secret));
    client.update_default_option(BybitOption::RecvWindow(3000));

    let cancel_all: serde_json::Value = client.post(
        "/futures/private/order/cancelAll",
        Some(json!({"symbol": "BTCUSDT"})),
        [BybitOption::HttpAuth(BybitHttpAuth::BelowV3)],
    ).await.expect("failed to cancel orders");
    println!("Cancel all result:\n{}", cancel_all);
}
