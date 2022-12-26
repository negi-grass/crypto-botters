# crypto-botters
仮想通貨取引所のAPIと通信するためのRustライブラリです。

[crates.io](https://crates.io/crates/crypto-botters)

詳しくは examples/ や[この記事](https://qiita.com/negi_grass/items/dc67d0af0d7b8d1b5d78)を見てください。

## 使い方

Cargo.toml:
```
[dependencies]
crypto-botters = { version = "0.2", features = ["binance", "bitflyer"] }
```

## 対応API
以下のAPIは最新バージョンで対応しています。
- [Binance](https://www.binance.com/en)
  - [Spot/Margin/Savings/Mining](https://binance-docs.github.io/apidocs/spot/en/)
  - [USDⓈ-M Futures](https://binance-docs.github.io/apidocs/futures/en/)
  - [COIN-M Futures](https://binance-docs.github.io/apidocs/delivery/en/)
  - [European Options](https://binance-docs.github.io/apidocs/voptions/en/)
  - [WebSocket API](https://binance-docs.github.io/apidocs/websocket_api/en/)
- [bitFlyer](https://bitflyer.com)
  - [HTTP API](https://lightning.bitflyer.com/docs)
  - [Realtime API](https://bf-lightning-api.readme.io/docs) (Socket.IO は非対応)

以下のAPIは次のバージョンで対応する予定です。
- [Bybit](https://www.bybit.com)
