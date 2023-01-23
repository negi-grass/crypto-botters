use std::env;
use log::LevelFilter;
use serde_json::json;
use crypto_botters::{
    Client,
    coincheck::{CoincheckOption},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("COINCHECK_API_KEY").expect("no API key found");
    let secret = env::var("COINCHECK_API_SECRET").expect("no API secret found");
    let mut client = Client::new();
    client.update_default_option(CoincheckOption::Key(key));
    client.update_default_option(CoincheckOption::Secret(secret));

    // https://coincheck.com/ja/documents/exchange/api#order-opens
    let open_orders: serde_json::Value = client.get_no_query(
        "/api/exchange/orders/opens",
        [CoincheckOption::HttpAuth(true)],
    ).await.expect("failed get orders");
    println!("open orders:\n{}", open_orders);

    // https://coincheck.com/ja/documents/exchange/api#order-new
    let order_result: serde_json::Value = client.post(
        "/api/exchange/orders",
        Some(json!({ "pair": "btc_jpy", "order_type": "market_buy", "market_buy_amount": 10000 })),
        [CoincheckOption::HttpAuth(true)],
    ).await.expect("failed to make order");
    println!("order result:\n{}", order_result);
}
