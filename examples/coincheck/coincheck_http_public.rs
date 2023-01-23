use log::LevelFilter;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crypto_botters::{
    Client,
    coincheck::{CoincheckOption},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    // typed
    #[derive(Serialize)]
    struct TickerParams<'a> {
        pair: &'a str,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct Ticker {
        #[serde(with = "rust_decimal::serde::float")]
        last: Decimal,
        #[serde(with = "rust_decimal::serde::float")]
        bid: Decimal,
        #[serde(with = "rust_decimal::serde::float")]
        ask: Decimal,
        #[serde(with = "rust_decimal::serde::float")]
        high: Decimal,
        #[serde(with = "rust_decimal::serde::float")]
        low: Decimal,
        #[serde(with = "rust_decimal::serde::float")]
        volume: Decimal,
        timestamp: i64,
    }

    // https://coincheck.com/ja/documents/exchange/api#ticker
    let ticker: Ticker = client.get(
        "/api/ticker",
        Some(&TickerParams { pair: "etc_jpy" }),
        [CoincheckOption::Default],
    ).await.expect("failed to get ticker");
    println!("ETC ticker:\n{:?}", ticker);

    // not typed
    // https://coincheck.com/ja/documents/exchange/api#order-book
    let orderbook: serde_json::Value = client.get(
        "/api/order_books",
        Some(&json!({ "pair": "etc_jpy" })),
        [CoincheckOption::Default],
    ).await.expect("failed get orderbook");
    println!("ETC asks:\n{}", orderbook["asks"]);
}
