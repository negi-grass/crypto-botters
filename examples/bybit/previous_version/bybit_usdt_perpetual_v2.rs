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

    // public GET
    let funding_rate: serde_json::Value = client.get(
        "/public/linear/funding/prev-funding-rate",
        Some(&[("symbol", "BTCUSDT")]),
        [BybitOption::HttpAuth(BybitHttpAuth::None)],
    ).await.expect("failed to get funding rate");
    println!("Funding rate:\n{funding_rate}");

    // private GET
    let risk_limit: serde_json::Value = client.get(
        "/public/linear/risk-limit",
        Some(&[("symbol", "BTCUSDT")]),
        [BybitOption::HttpAuth(BybitHttpAuth::BelowV3)],
    ).await.expect("failed to get risk limit");
    println!("Risk limit:\n{risk_limit}");

    // private POST
    let cancel_result: serde_json::Value = client.post(
        "/private/linear/order/cancel-all",
        Some(json!({"symbol": "BTCUSDT"})),
        [BybitOption::HttpAuth(BybitHttpAuth::BelowV3)],
    ).await.expect("failed to cancel orders");
    println!("Cancel result:\n{cancel_result}");
}
