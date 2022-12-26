use log::LevelFilter;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crypto_botters::{
    http::Client,
    binance::{Binance, BinanceSecurity, BinanceHttpUrl},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let binance = Binance::new(None, None);
    let client = Client::new();

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
        &binance.request(BinanceSecurity::None, BinanceHttpUrl::Spot),
    ).await.expect("failed to get tickers");
    println!("BTC & ETH prices:\n{:?}", ticker.price);

    // not typed
    let orderbook: serde_json::Value = client.get(
        "https://api.binance.com/api/v3/ticker/bookTicker",
        Some(&json!({ "symbol": "BTCUSDT" })),
        &binance.request_no_url(BinanceSecurity::None),
    ).await.expect("failed get orderbook");
    println!("BTC bidPrice:\n{:?}", orderbook["bidPrice"]);
}
