use std::time::Duration;
use log::LevelFilter;
use crypto_botters::{Client, coincheck::CoincheckOption};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let client = Client::new();

    let connection = client.websocket(
        "/",
        |message| println!("{}", message),
        [CoincheckOption::WebSocketChannels(vec!["btc_jpy-orderbook".to_owned()])],
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(5)).await;

    // manually reconnect
    connection.reconnect_state().request_reconnect();

    // receive messages. we should see no missing message during reconnection
    tokio::time::sleep(Duration::from_secs(5)).await;

    // close the connection
    drop(connection);

    // wait for the "close" message to be logged
    tokio::time::sleep(Duration::from_secs(1)).await;
}
