use std::{
    time::Duration,
    env,
};
use log::LevelFilter;
use crypto_botters::{Client, bybit::{BybitOption, BybitWebSocketUrl}};

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

    // https://bybit-exchange.github.io/docs/spot/v3/#t-websockettrade
    let connection = client.websocket(
        "/spot/private/v3",
        |message| println!("{}", message),
        [
            BybitOption::WebSocketTopics(vec!["order".to_owned()]),
            BybitOption::WebSocketUrl(BybitWebSocketUrl::Bytick),
            BybitOption::WebSocketAuth(true),
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
