# crypto-botters
[![](https://img.shields.io/crates/v/crypto-botters)](https://crates.io/crates/crypto-botters)
[![](https://docs.rs/crypto-botters/badge.svg)](https://docs.rs/crypto-botters)
[![](https://github.com/negi-grass/crypto-botters/actions/workflows/clippy.yml/badge.svg)](https://github.com/negi-grass/crypto-botters/actions/workflows/clippy.yml)

[日本語は下にあります](#%E6%97%A5%E6%9C%AC%E8%AA%9E)

This is a Rust library for communicating with cryptocurrency exchange APIs.

This library:
- is asynchronous
- supports WebSocket
- supports deserializing responses into user-defined types

## Supported APIs
The following APIs are currently supported.
- [Binance](https://www.binance.com/en) ([Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/binance))
  - [Spot/Margin/Savings/Mining](https://binance-docs.github.io/apidocs/spot/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams))
  - [USDⓈ-M Futures](https://binance-docs.github.io/apidocs/futures/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams))
  - [COIN-M Futures](https://binance-docs.github.io/apidocs/delivery/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/delivery/en/#websocket-market-streams))
  - [European Options](https://binance-docs.github.io/apidocs/voptions/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/voptions/en/#websocket-market-streams))
  - [WebSocket API](https://binance-docs.github.io/apidocs/websocket_api/en/)
- [bitFlyer](https://bitflyer.com) ([Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/bitflyer))
  - [HTTP API](https://lightning.bitflyer.com/docs)
  - [Realtime API](https://bf-lightning-api.readme.io/docs) (Socket.IO not supported)
- [Bybit](https://www.bybit.com/en-US/) ([Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/bybit))
  - [Derivatives v3 Unified Margin](https://bybit-exchange.github.io/docs/derivativesV3/unified_margin/)
    ([WebSocket](https://bybit-exchange.github.io/docs/derivativesV3/unified_margin/#t-websocket))
  - [Derivatives v3 Contract](https://bybit-exchange.github.io/docs/derivativesV3/contract/)
    ([WebSocket](https://bybit-exchange.github.io/docs/derivativesV3/contract/#t-websocket))
  - [Futures v2 Inverse Perpetual](https://bybit-exchange.github.io/docs/futuresV2/inverse/)
    ([WebSocket](https://bybit-exchange.github.io/docs/futuresV2/inverse/#t-websocket))
  - [Futures v2 USDT Perpetual](https://bybit-exchange.github.io/docs/futuresV2/linear/)
    ([WebSocket](https://bybit-exchange.github.io/docs/futuresV2/linear/#t-websocket))
  - [Futures v2 Inverse Futures](https://bybit-exchange.github.io/docs/futuresV2/inverse_futures/)
    ([WebSocket](https://bybit-exchange.github.io/docs/futuresV2/inverse_futures/#t-websocket))
  - [Spot v3](https://bybit-exchange.github.io/docs/spot/v3/)
    ([WebSocket](https://bybit-exchange.github.io/docs/spot/v3/#t-websocket))
  - [Spot v1](https://bybit-exchange.github.io/docs/spot/v1/)
    ([WebSocket](https://bybit-exchange.github.io/docs/spot/v1/#t-websocket))
  - [Account Asset v3](https://bybit-exchange.github.io/docs/account_asset/v3/)
  - [Account Asset v1](https://bybit-exchange.github.io/docs/account_asset/v1/)
  - [Copy Trading](https://bybit-exchange.github.io/docs/copy_trading/)
    ([WebSocket](https://bybit-exchange.github.io/docs/copy_trading/#t-websocket))
  - [USDC Contract Option](https://bybit-exchange.github.io/docs/usdc/option/)
    ([WebSocket](https://bybit-exchange.github.io/docs/usdc/option/#t-websocket))
  - [USDC Contract Perpetual](https://bybit-exchange.github.io/docs/usdc/perpetual/)
    ([WebSocket](https://bybit-exchange.github.io/docs/usdc/perpetual/#t-websocket))
  - [Tax](https://bybit-exchange.github.io/docs/tax/)
- [Coincheck](https://coincheck.com/ja/documents/exchange/api) ([Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/coincheck))
  - [Public API](https://coincheck.com/ja/documents/exchange/api#public)
  - [Private API](https://coincheck.com/ja/documents/exchange/api#private)
  - [WebSoket API](https://coincheck.com/ja/documents/exchange/api#websocket)

## Usage
More than 20 examples can be found in the [examples directory](https://github.com/negi-grass/crypto-botters/tree/main/examples).

Cargo.toml:
```
[dependencies]
crypto-botters = { version = "0.4", features = ["binance", "bitflyer", "bybit", "coincheck"] }
```
Enable the features for the exchanges that you use.

## Example
### HTTP
```rust
use std::env;
use crypto_botters::{Client, binance::{BinanceAuth, BinanceHttpUrl, BinanceOption}};

#[tokio::main]
async fn main() {
  let key = env::var("BINANCE_API_KEY").expect("no API key found");
  let secret = env::var("BINANCE_API_SECRET").expect("no API secret found");
  let mut client = Client::new();
  client.update_default_option(BinanceOption::Key(key));
  client.update_default_option(BinanceOption::Secret(secret));
  
  let dusts: serde_json::Value = client.post_no_body(
    "https://api.binance.com/sapi/v1/asset/dust-btc",
    [BinanceOption::HttpAuth(BinanceAuth::Sign)],
  ).await.expect("failed get dusts");
  println!("My dust assets(BTC):\n{:?}", dusts["totalTransferBtc"]);
}
```
The above code queries assets that are convertable into BNB using the Binance API.

### Options
When making a request, you pass some options to, for example, the `post_no_body` function.
In the example, `[BinanceOption::HttpAuth(BinanceAuth::Sign)]` is the options.
You would usually pass an **array of options**.

The options are for:
- setting API key/secret
- enabling authentication

etc.

The type of options passed is what determines the exchange used. In the above example, the library knows
the request is for Binance because the type of the option passed is `BinanceOption`. When using Bybit,
you would pass an array of `BybitOption`s.

### Default Options
Some options are the same across requests. For example, you will probably use the same API key for each request.
For those options, you can set **default options** for `Client`. Default options are applied to all requests.

In the above example, `client.update_default_option(BinanceOption::Key(key));` sets the option for Binance API key as a default option.
Because of this, passing an option for API key in `post_no_body()` is not required.

### Response type
Responses are automatically deserialized into the specified type. In the above example, the response is of the type `serde_json::Value`
because we specified the type of `dusts`. Any type that implements `DeserializeOwned` is supported.

### WebSocket
```rust
use std::time::Duration;
use log::LevelFilter;
use crypto_botters::{binance::{BinanceOption, BinanceWebSocketUrl}, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let connection = client.websocket(
        "/ws/btcusdt@trade",
        |message| println!("{}", message),
        [BinanceOption::WebSocketUrl(BinanceWebSocketUrl::Spot443)],
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(10)).await;
}
```
The above code opens a WebSocket connection and watches BTCUSDT trades happening on Binance.

The `Client::websocket()` method returns a `WebSocketConnection`. Using this, you can send messages,
request a reconnection, or close the connection.

## 日本語
これは仮想通貨取引所のAPIと通信するためのRustライブラリです。

特徴:
- 非同期
- WebSocketに対応
- レスポンスをユーザーの定義した型に変換

## 対応API
以下のAPIに対応しています。
- [Binance](https://www.binance.com/en) ([例](https://github.com/negi-grass/crypto-botters/tree/main/examples/binance))
  - [Spot/Margin/Savings/Mining](https://binance-docs.github.io/apidocs/spot/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams))
  - [USDⓈ-M Futures](https://binance-docs.github.io/apidocs/futures/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams))
  - [COIN-M Futures](https://binance-docs.github.io/apidocs/delivery/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/delivery/en/#websocket-market-streams))
  - [European Options](https://binance-docs.github.io/apidocs/voptions/en/)
    ([WebSocket](https://binance-docs.github.io/apidocs/voptions/en/#websocket-market-streams))
  - [WebSocket API](https://binance-docs.github.io/apidocs/websocket_api/en/)
- [bitFlyer](https://bitflyer.com) ([例](https://github.com/negi-grass/crypto-botters/tree/main/examples/bitflyer))
  - [HTTP API](https://lightning.bitflyer.com/docs)
  - [Realtime API](https://bf-lightning-api.readme.io/docs) (Socket.IO は非対応)
- [Bybit](https://www.bybit.com/en-US/) ([例](https://github.com/negi-grass/crypto-botters/tree/main/examples/bybit))
  - [Derivatives v3 Unified Margin](https://bybit-exchange.github.io/docs/derivativesV3/unified_margin/)
    ([WebSocket](https://bybit-exchange.github.io/docs/derivativesV3/unified_margin/#t-websocket))
  - [Derivatives v3 Contract](https://bybit-exchange.github.io/docs/derivativesV3/contract/)
    ([WebSocket](https://bybit-exchange.github.io/docs/derivativesV3/contract/#t-websocket))
  - [Futures v2 Inverse Perpetual](https://bybit-exchange.github.io/docs/futuresV2/inverse/)
    ([WebSocket](https://bybit-exchange.github.io/docs/futuresV2/inverse/#t-websocket))
  - [Futures v2 USDT Perpetual](https://bybit-exchange.github.io/docs/futuresV2/linear/)
    ([WebSocket](https://bybit-exchange.github.io/docs/futuresV2/linear/#t-websocket))
  - [Futures v2 Inverse Futures](https://bybit-exchange.github.io/docs/futuresV2/inverse_futures/)
    ([WebSocket](https://bybit-exchange.github.io/docs/futuresV2/inverse_futures/#t-websocket))
  - [Spot v3](https://bybit-exchange.github.io/docs/spot/v3/)
    ([WebSocket](https://bybit-exchange.github.io/docs/spot/v3/#t-websocket))
  - [Spot v1](https://bybit-exchange.github.io/docs/spot/v1/)
    ([WebSocket](https://bybit-exchange.github.io/docs/spot/v1/#t-websocket))
  - [Account Asset v3](https://bybit-exchange.github.io/docs/account_asset/v3/)
  - [Account Asset v1](https://bybit-exchange.github.io/docs/account_asset/v1/)
  - [Copy Trading](https://bybit-exchange.github.io/docs/copy_trading/)
    ([WebSocket](https://bybit-exchange.github.io/docs/copy_trading/#t-websocket))
  - [USDC Contract Option](https://bybit-exchange.github.io/docs/usdc/option/)
    ([WebSocket](https://bybit-exchange.github.io/docs/usdc/option/#t-websocket))
  - [USDC Contract Perpetual](https://bybit-exchange.github.io/docs/usdc/perpetual/)
    ([WebSocket](https://bybit-exchange.github.io/docs/usdc/perpetual/#t-websocket))
  - [Tax](https://bybit-exchange.github.io/docs/tax/)
- [Coincheck](https://coincheck.com/ja/documents/exchange/api) ([例](https://github.com/negi-grass/crypto-botters/tree/main/examples/coincheck))
  - [Public API](https://coincheck.com/ja/documents/exchange/api#public)
  - [Private API](https://coincheck.com/ja/documents/exchange/api#private)
  - [WebSoket API](https://coincheck.com/ja/documents/exchange/api#websocket)
## 使い方
[examples ディレクトリ](https://github.com/negi-grass/crypto-botters/tree/main/examples) にサンプルが20以上あります。

Cargo.toml:
```
[dependencies]
crypto-botters = { version = "0.4", features = ["binance", "bitflyer", "bybit", "coincheck"] }
```
使いたい取引所のfeatureを有効化してください。

## 例
### HTTP
```rust
use std::env;
use crypto_botters::{Client, binance::{BinanceAuth, BinanceHttpUrl, BinanceOption}};

#[tokio::main]
async fn main() {
  let key = env::var("BINANCE_API_KEY").expect("no API key found");
  let secret = env::var("BINANCE_API_SECRET").expect("no API secret found");
  let mut client = Client::new();
  client.update_default_option(BinanceOption::Key(key));
  client.update_default_option(BinanceOption::Secret(secret));
  
  let dusts: serde_json::Value = client.post_no_body(
    "https://api.binance.com/sapi/v1/asset/dust-btc",
    [BinanceOption::HttpAuth(BinanceAuth::Sign)],
  ).await.expect("failed get dusts");
  println!("My dust assets(BTC):\n{:?}", dusts["totalTransferBtc"]);
}
```
この例では、BinanceでBNBに変換できる資産を取得しています。

### オプション
リクエストを送るときには、オプションを設定できます。この例では、`[BinanceOption::HttpAuth(BinanceAuth::Sign)]`
がオプションです。
普通はオプションの配列として渡します。

オプションは
- APIキーやシークレットを指定する
- 認証を有効にする

などのために設定します。

オプションの型がどの取引所を使うかを定めます。この例では`BinanceOption`型を渡しているため、Binance用に認証アルゴリズムが
用いられます。`BybitOption`を渡せばBybitへのリクエストとして扱われます。

### デフォルトオプション
複数のリクエスト間で変わらないオプションもあります。例えば、すべてのリクエストで同じAPIキーを使うことが多いと思います。
そのようなオプションは**デフォルトオプション**として`Client`に設定できます。デフォルトオプションは、その`Client`を
使って送られるすべてのリクエストに適用されます。

この例では、`client.update_default_option(BinanceOption::Key(key));`でAPIキーのオプションをデフォルトオプションとして設定
しています。このため、`post_no_body()`にAPIキーのオプションを指定する必要がなくなっています。

### レスポンスの型
レスポンスは指定した型に自動的に変換されます。この例では、`dusts`の型を`serde_json::Value`と指定しているため、
レスポンスが自動で`serde_json::Value`型に変換されています。`DeserializeOwned`を実装している肩ならどんな型でも指定できます。

### WebSocket
```rust
use std::time::Duration;
use log::LevelFilter;
use crypto_botters::{binance::{BinanceOption, BinanceWebSocketUrl}, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let connection = client.websocket(
        "/ws/btcusdt@trade",
        |message| println!("{}", message),
        [BinanceOption::WebSocketUrl(BinanceWebSocketUrl::Spot443)],
    ).await.expect("failed to connect websocket");
    // receive messages
    tokio::time::sleep(Duration::from_secs(10)).await;
}
```
この例では、BinanceのBTCUSDTの取引をリアルタイムで受信しています。

`Client::websocket()`メソッドは`WebSocketConnection`を返します。これに対し、メッセージを送信する、再接続を要求する、接続を
切断するなどの処理が行なえます。

## その他
開発者：[@negi_grass](https://twitter.com/negi_grass)
