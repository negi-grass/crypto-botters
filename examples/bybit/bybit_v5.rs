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
    client.update_default_option(BybitOption::Key(key));
    client.update_default_option(BybitOption::Secret(secret));

    // private POST
    let upgrade: serde_json::Value = client.post_no_body(
        "/v5/account/upgrade-to-uta",
        [BybitOption::HttpAuth(BybitHttpAuth::V3AndAbove)],
    ).await.expect("failed to upgrade account");
    println!("Upgrade result:\n{upgrade}");

    // private GET
    let coins: serde_json::Value = client.get(
        "/v5/asset/transfer/query-account-coins-balance",
        Some(&[("accountType", "UNIFIED")]),
        [BybitOption::HttpAuth(BybitHttpAuth::V3AndAbove)],
    ).await.expect("failed to get coins");
    println!("Coins:\n{coins}");

    // public GET
    let ticker: serde_json::Value = client.get(
        "/v5/market/tickers",
        Some(&[("symbol", "BTCUSDT")]),
        [BybitOption::Default],
    ).await.expect("failed to get ticker");
    println!("Ticker:\n{ticker}");
}
