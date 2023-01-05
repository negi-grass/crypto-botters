use log::LevelFilter;
use serde::Serialize;
use crypto_botters::{Client, bybit::BybitOption};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    // typed parameter, public
    #[derive(Serialize)]
    struct SymbolInfoParams<'a> {
        category: &'a str,
        symbol: &'a str,
    }

    let symbol_info: serde_json::Value = client.get(
        "/derivatives/v3/public/tickers",
        Some(&SymbolInfoParams { category: "linear", symbol: "BTCUSDT" }),
        [BybitOption::Default],
    ).await.expect("failed get symbol info");
    println!("Unified margin BTCUSDT info:\n{}", symbol_info);
}
