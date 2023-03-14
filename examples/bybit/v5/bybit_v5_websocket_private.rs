use std::{env, time::Duration};
use log::LevelFilter;
use crypto_botters::{Client, bybit::BybitOption};

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

    let connection = client.websocket(
        "/v5/private",
        |message| println!("{}", message),
        [
            BybitOption::WebSocketTopics(vec!["wallet".to_owned()]),
            BybitOption::WebSocketAuth(true),
        ],
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(300)).await;

    // close the connection
    drop(connection);

    // wait for the "close" message to be logged
    tokio::time::sleep(Duration::from_secs(1)).await;
}
