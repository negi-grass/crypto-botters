use std::env;
use log::LevelFilter;
use serde::Serialize;
use serde_json::json;
use crypto_botters::{
    http::Client,
    bitflyer::{BitFlyer, BitflyerSecurity},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BITFLYER_API_KEY").expect("no API key found");
    let secret = env::var("BITFLYER_API_SECRET").expect("no API secret found");
    let bitflyer = BitFlyer::new(Some(key), Some(secret));
    let client = Client::new();

    // typed
    #[derive(Serialize)]
    struct CancelOrderParams<'a> {
        product_code: &'a str,
        child_order_id: &'a str,
    }

    // will return ParseError. https://github.com/serde-rs/serde/issues/1425
    let result: Result<(), _> = client.post(
        "/v1/me/cancelchildorder",
        Some(&CancelOrderParams { product_code: "FX_BTC_JPY", child_order_id: "JOR20150707-055555-022222" }), // example id
        &bitflyer.request(BitflyerSecurity::Sign),
    ).await;
    println!("Cancel order result:\n{:?}", result);

    // not typed
    let commission: serde_json::Value = client.get(
        "/v1/me/gettradingcommission",
        Some(&json!({ "product_code": "BTC_JPY" })),
        &bitflyer.request(BitflyerSecurity::Sign),
    ).await.expect("failed get commission");
    println!("commission rate:\n{:?}", commission["commission_rate"]);
}
