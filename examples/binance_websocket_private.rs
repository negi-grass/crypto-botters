use std::{
    env,
    time::Duration,
};
use log::LevelFilter;
use crypto_botters::{
    http::Client,
    websocket::WebSocketConnection,
    binance::{Binance, BinanceWebSocketUrl},
};
use crypto_botters_binance::{BinanceHttpUrl, BinanceSecurity};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BINANCE_API_KEY").expect("no API key found");
    let secret = env::var("BINANCE_API_SECRET").expect("no API secret found");
    let binance = Binance::new(Some(key), Some(secret));
    let client = Client::new();

    let key: serde_json::Value = client.post(
        "/sapi/v1/userDataStream/isolated",
        Some(&[("symbol", "BTCUSDT")]),
        &binance.request(BinanceSecurity::Key, BinanceHttpUrl::Spot),
    ).await.expect("failed to get listen key");

    let _connection = WebSocketConnection::new(
        &format!("/ws/{}", key["listenKey"].as_str().unwrap()),
        binance.websocket(|message| {
            println!("{}", message);
        }, BinanceWebSocketUrl::Stream9443)
    ).await.expect("failed to connect websocket");

    // receive messages
    tokio::time::sleep(Duration::from_secs(60)).await;
}
