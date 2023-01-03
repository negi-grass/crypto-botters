use log::LevelFilter;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crypto_botters::{
    Client,
    binance::{BinanceHttpUrl, BinanceOption},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let mut client = Client::new();
    client.default_option(BinanceOption::HttpUrl(BinanceHttpUrl::Spot));

    // typed
    #[derive(Serialize)]
    struct TickerParams<'a> {
        symbol: &'a str,
    }

    #[derive(Deserialize)]
    struct Ticker {
        #[serde(with = "rust_decimal::serde::str")]
        price: Decimal,
        #[allow(dead_code)]
        symbol: String,
    }

    let ticker: Ticker = client.get(
        "/api/v3/ticker/price",
        Some(&TickerParams { symbol: "BTCUSDT" }),
        [BinanceOption::Default],
    ).await.expect("failed to get tickers");
    println!("BTC & ETH prices:\n{:?}", ticker.price);

    // not typed
    let orderbook: serde_json::Value = client.get(
        "https://api.binance.com/api/v3/ticker/bookTicker",
        Some(&json!({ "symbol": "BTCUSDT" })),
        [BinanceOption::HttpUrl(BinanceHttpUrl::None)],
    ).await.expect("failed get orderbook");
    println!("BTC bidPrice:\n{:?}", orderbook["bidPrice"]);
}
