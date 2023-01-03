use log::LevelFilter;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crypto_botters::{
    Client,
    bitflyer::{BitFlyerOption},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    // typed
    #[derive(Serialize)]
    struct ExecutionsParams<'a> {
        product_code: &'a str,
        count: u32,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct Execution {
        id: i64,
        side: String,
        #[serde(with = "rust_decimal::serde::float")]
        price: Decimal,
        #[serde(with = "rust_decimal::serde::float")]
        size: Decimal,
        exec_date: String,
        buy_child_order_acceptance_id: String,
        sell_child_order_acceptance_id: String,
    }

    let executions: Vec<Execution> = client.get(
        "/v1/executions",
        Some(&ExecutionsParams { product_code: "FX_BTC_JPY", count: 10 }),
        [BitFlyerOption::Default],
    ).await.expect("failed to get executions");
    println!("BTC executions:\n{:?}", executions);

    // not typed
    let orderbook: serde_json::Value = client.get(
        "/v1/board",
        Some(&json!({ "product_code": "FX_BTC_JPY" })),
        [BitFlyerOption::Default],
    ).await.expect("failed get orderbook");
    println!("BTC mid price:\n{:?}", orderbook["mid_price"]);
}
