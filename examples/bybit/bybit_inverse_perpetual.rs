use std::env;
use log::LevelFilter;
use crypto_botters::{Client, bybit::{BybitOption}};
use crypto_botters_bybit::BybitHttpAuth;

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
    client.update_default_option(BybitOption::RecvWindow(6000));

    let open_interest: serde_json::Value = client.get(
        "/v2/public/open-interest",
        Some(&[("symbol", "BTCUSDT"), ("period", "1d"), ("limit", "3")]),
        [BybitOption::HttpAuth(BybitHttpAuth::None)],
    ).await.expect("failed to cancel orders");
    println!("Open interest:\n{}", open_interest);

    let positions: serde_json::Value = client.get(
        "/v2/private/position/list",
        Some(&[("symbol", "BTCUSDT")]),
        [BybitOption::HttpAuth(BybitHttpAuth::Type1)],
    ).await.expect("failed to get positions");
    println!("Positions:\n{}", positions);
}
