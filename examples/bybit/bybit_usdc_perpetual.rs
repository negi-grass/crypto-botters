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

    // private POST
    let cancel_all: serde_json::Value = client.post(
        "/perpetual/usdc/openapi/private/v1/cancel-all",
            Some(json!({"symbol": "BTCPERP", "orderFilter": "Order"})),
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to cancel orders");
    println!("Cancel all result:\n{}", cancel_all);

    // private POST
    let open_orders: serde_json::Value = client.post(
        "/option/usdc/openapi/private/v1/query-active-orders",
            Some(json!({"category": "PERPETUAL"})),
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to get orders");
    println!("Open orders:\n{}", open_orders);

    // public GET
    let symbol_info: serde_json::Value = client.get(
        "/perpetual/usdc/openapi/public/v1/tick",
        Some(&[("symbol", "BTCPERP")]),
        [BybitOption::Default],
    ).await.expect("failed to get symbol info");
    println!("Symbol info:\n{}", symbol_info);
}
