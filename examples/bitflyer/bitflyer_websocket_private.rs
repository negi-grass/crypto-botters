use std::{
    env,
    time::Duration,
};
use log::LevelFilter;
use crypto_botters::{Client, bitflyer::BitFlyerOption};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BITFLYER_API_KEY").expect("no API key found");
    let secret = env::var("BITFLYER_API_SECRET").expect("no API secret found");
    let mut client = Client::new();
    client.default_option(BitFlyerOption::Key(key));
    client.default_option(BitFlyerOption::Secret(secret));

    let connection = client.websocket(
        "/json-rpc",
        |message| println!("{:?}", message),
        [
            BitFlyerOption::WebSocketChannels(vec!["child_order_events".to_owned()]),
            BitFlyerOption::WebSocketAuth(true),
        ],
    ).await.expect("failed to connect websocket");

    // receive messages
    tokio::time::sleep(Duration::from_secs(10)).await;

    // reconnect
    connection.reconnect_state().request_reconnect();

    // we should still see private channel messages
    tokio::time::sleep(Duration::from_secs(10)).await;
}
