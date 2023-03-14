# crypto-botters
[![](https://img.shields.io/crates/v/crypto-botters)](https://crates.io/crates/crypto-botters)
[![](https://docs.rs/crypto-botters/badge.svg)](https://docs.rs/crypto-botters)
[![](https://github.com/negi-grass/crypto-botters/actions/workflows/clippy.yml/badge.svg)](https://github.com/negi-grass/crypto-botters/actions/workflows/clippy.yml)
[![](https://img.shields.io/github/license/negi-grass/crypto-botters)](https://github.com/negi-grass/crypto-botters/blob/main/LICENSE)

[日本語は下にあります](#%E6%97%A5%E6%9C%AC%E8%AA%9E)

This is a Rust library for communicating with cryptocurrency exchange APIs.

This library:
- is asynchronous
- supports WebSocket
- supports deserializing responses into user-defined types

## Supported Exchanges
The following Exchanges are currently supported.

| Exchange                           | Official API document                                           | Example usages of this library                                                        |
|------------------------------------|-----------------------------------------------------------------|---------------------------------------------------------------------------------------|
| [Binance](https://www.binance.com) | [API document](https://binance-docs.github.io/apidocs/spot/en/) | [Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/binance)   |
| [bitFlyer](https://bitflyer.com)   | [API document](https://lightning.bitflyer.com/docs)             | [Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/bitflyer)  |
| [Bybit](https://www.bybit.com)     | [API document](https://bybit-exchange.github.io/docs/)          | [Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/bybit)     |
| [Coincheck](https://coincheck.com) | [API document](https://coincheck.com/documents/exchange/api)    | [Examples](https://github.com/negi-grass/crypto-botters/tree/main/examples/coincheck) |

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

## 対応取引所
以下の取引所に対応しています。

| 取引所名                               | 公式APIドキュメント                                                  | 本ライブラリ使用例                                                                        |
|------------------------------------|--------------------------------------------------------------|----------------------------------------------------------------------------------|
| [Binance](https://www.binance.com) | [APIドキュメント](https://binance-docs.github.io/apidocs/spot/en/) | [使用例](https://github.com/negi-grass/crypto-botters/tree/main/examples/binance)   |
| [bitFlyer](https://bitflyer.com)   | [APIドキュメント](https://lightning.bitflyer.com/docs)             | [使用例](https://github.com/negi-grass/crypto-botters/tree/main/examples/bitflyer)  |
| [Bybit](https://www.bybit.com)     | [APIドキュメント](https://bybit-exchange.github.io/docs/)          | [使用例](https://github.com/negi-grass/crypto-botters/tree/main/examples/bybit)     |
| [Coincheck](https://coincheck.com) | [APIドキュメント](https://coincheck.com/ja/documents/exchange/api) | [使用例](https://github.com/negi-grass/crypto-botters/tree/main/examples/coincheck) |

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
リクエストを送るときには、オプションを設定できます。この例では、`[BinanceOption::HttpAuth(BinanceAuth::Sign)]`がオプションです。
オプションのイテレータなら何でもいいです。この例では配列を使っています。

オプションは
- APIキーやシークレットを指定する
- 認証を有効にする

などのために設定します。

オプションの型がどの取引所を使うかを定めます。この例では`BinanceOption`型を渡しているため、Binanceの認証アルゴリズムが用いられます。`BybitOption`型を渡せばBybitへのリクエストとして扱われます。

### デフォルトオプション
複数のリクエスト間で変わらないオプションもあります。例えば、すべてのリクエストで同じAPIキーを使うことが多いと思います。
そのようなオプションは**デフォルトオプション**として`Client`に設定できます。デフォルトオプションは、その`Client`を
使って送られるすべてのリクエストに適用されます。それぞれのリクエストで渡すオプションで上書きすることもできます。

この例では、`client.update_default_option(BinanceOption::Key(key));`でAPIキーのオプションをデフォルトオプションとして設定
しています。このため、`post_no_body()`にAPIキーのオプションを指定する必要がなくなっています。

### レスポンスの型
レスポンスは指定した型に自動的に変換されます。この例では、`dusts`の型を`serde_json::Value`と指定しているため、
レスポンスが自動で`serde_json::Value`型に変換されています。`DeserializeOwned`を実装している型ならどんな型でも指定できます。

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

`Client::websocket()`メソッドは`WebSocketConnection`型を返します。これに対し、メッセージを送信する、再接続を要求する、接続を切断するなどの処理が行なえます。

## その他
開発者：[@negi_grass](https://twitter.com/negi_grass)
