use log::LevelFilter;
use serde_json::json;
use crypto_botters::{
    Client,
    binance::{BinanceOption, BinanceRequestResult},
};

struct Api {
    client: Client,
}

impl Api {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn get_orderbook(&self) -> BinanceRequestResult<serde_json::Value> {
        self.client.get(
            "https://api.binance.com/api/v3/ticker/bookTicker",
            Some(&json!({ "symbol": "BTCUSDT" })),
            [BinanceOption::Default],
        ).await
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let api = Api::new();
    let handle = tokio::spawn(async move {
        log::info!("in async block");
        let orderbook = api.get_orderbook().await;
        println!("{:?}", orderbook);
    });
    let result = handle.await;
    println!("{:?}", result);
}
