use std::time::Duration;
use log::LevelFilter;
use crypto_botters::{Client, bybit::{BybitOption, BybitWebSocketUrl}};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    // https://bybit-exchange.github.io/docs/spot/v3/#t-websockettrade
    let connection = client.websocket(
        "/spot/public/v3",
        |message| println!("{}", message),
        [
            BybitOption::WebSocketTopics(vec!["trade.BTCUSDT".to_owned()]),
            BybitOption::WebSocketUrl(BybitWebSocketUrl::Bytick),
        ],
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(5)).await;

    // manually reconnect
    connection.reconnect_state().request_reconnect();

    // receive messages. there should be no missing or duplicate messages during reconnection
    tokio::time::sleep(Duration::from_secs(5)).await;

    // close the connection
    drop(connection);

    // wait for the "close" message to be logged
    tokio::time::sleep(Duration::from_secs(1)).await;
}
