use std::time::Duration;
use log::LevelFilter;
use crypto_botters::{
    websocket::WebSocketConnection,
    binance::{Binance, BinanceWebSocketUrl},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let binance = Binance::new(None, None);

    let connection = WebSocketConnection::new(
        "/ws/btcusdt@trade",
        binance.websocket(|message| {
            println!("{}", message);
        }, BinanceWebSocketUrl::Stream443)
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(2)).await;

    // manually reconnect
    connection.reconnect_state().request_reconnect();

    // receive messages. we should see no missing message during reconnection
    tokio::time::sleep(Duration::from_secs(2)).await;

    // close the connection
    drop(connection);

    // wait for the "close" message to be logged
    tokio::time::sleep(Duration::from_secs(1)).await;
}
