use std::{
    env,
    time::Duration,
};
use log::LevelFilter;
use crypto_botters::{
    websocket::WebSocketConnection,
    bitflyer::BitFlyer,
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BITFLYER_API_KEY").expect("no API key found");
    let secret = env::var("BITFLYER_API_SECRET").expect("no API secret found");
    let bitflyer = BitFlyer::new(Some(key), Some(secret));

    let connection = WebSocketConnection::new(
        "/json-rpc",
        bitflyer.websocket(|message| {
            println!("{:?}", message);
        }, vec!["child_order_events"], true),
    ).await.expect("failed to connect websocket");

    // receive messages
    tokio::time::sleep(Duration::from_secs(10)).await;

    // reconnect
    connection.reconnect_state().request_reconnect();

    // we should still see private channel messages
    tokio::time::sleep(Duration::from_secs(10)).await;
}
