use std::time::Duration;
use log::LevelFilter;
use crypto_botters::{binance::{BinanceOption, BinanceWebSocketUrl}, Client};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    let connection = client.websocket(
        "/ws/btcusdt@trade",
        |message| println!("{}", message),
        [BinanceOption::WebSocketUrl(BinanceWebSocketUrl::Spot443)],
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(1)).await;

    // manually reconnect
    connection.reconnect_state().request_reconnect();

    // receive messages. we should see no missing message during reconnection
    tokio::time::sleep(Duration::from_secs(3)).await;

    // close the connection
    drop(connection);

    // wait for the "close" message to be logged
    tokio::time::sleep(Duration::from_secs(1)).await;
}
