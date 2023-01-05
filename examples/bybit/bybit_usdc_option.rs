use std::env;
use log::LevelFilter;
use crypto_botters::{Client, bybit::{BybitOption, BybitHttpAuth}};

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
    let cancel_all: serde_json::Value = client.post_no_body(
        "/option/usdc/openapi/private/v1/cancel-all",
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to cancel orders");
    println!("Cancel all result:\n{}", cancel_all);

    // private GET
    let open_orders: serde_json::Value = client.get_no_query(
        "/option/usdc/openapi/private/v1/trade/query-active-orders",
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to get orders");
    println!("Open orders:\n{}", open_orders);

    // public GET
    let symbol_info: serde_json::Value = client.get(
        "/option/usdc/openapi/public/v1/tick",
        Some(&[("symbol", "BTC-5JAN23-18500-C")]),
        [BybitOption::Default],
    ).await.expect("failed to get symbol info");
    println!("Symbol info:\n{}", symbol_info);
}
