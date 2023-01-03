use std::{
    env,
    time::Duration,
};
use log::LevelFilter;
use crypto_botters::{
    Client,
    binance::{BinanceOption, BinanceWebSocketUrl},
};
use crypto_botters_binance::{BinanceHttpUrl, BinanceAuth};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BINANCE_API_KEY").expect("no API key found");
    let secret = env::var("BINANCE_API_SECRET").expect("no API secret found");
    let mut client = Client::new();
    client.default_option(BinanceOption::Key(key));
    client.default_option(BinanceOption::Secret(secret));

    let key: serde_json::Value = client.post(
        "/sapi/v1/userDataStream/isolated",
        Some(&[("symbol", "BTCUSDT")]),
        [BinanceOption::HttpAuth(BinanceAuth::Key), BinanceOption::HttpUrl(BinanceHttpUrl::Spot)],
    ).await.expect("failed to get listen key");

    let _connection = client.websocket(
        &format!("/ws/{}", key["listenKey"].as_str().unwrap()),
        |message| println!("{}", message),
        [BinanceOption::WebSocketUrl(BinanceWebSocketUrl::Spot9443)],
    ).await.expect("failed to connect websocket");

    // receive messages
    tokio::time::sleep(Duration::from_secs(60)).await;
}
