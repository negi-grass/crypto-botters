# Changelog

## Unreleased
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/61048cea6360d8ec047d29dccacc49a8f2e1771d...main)

## 0.3.0 (2023-12-07)
- [crates.io](https://crates.io/crates/generic-api-client/0.3.0)
- [docs.rs](https://docs.rs/generic-api-client/0.3.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/61048cea6360d8ec047d29dccacc49a8f2e1771d/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/9a2e1ac25587c3981e4348538ba99f3ed93b9817...61048cea6360d8ec047d29dccacc49a8f2e1771d)

### Added
- A new field `message_timeout` was added to `WebSocketConfig`, which enables users to set timeouts on message reception.

### Changed
- Send a close frame when user drops WebSocketConnection
- Automatically trigger a reconnection on server connection close
- Added some log calls

## 0.2.1 (2023-03-17)
- [crates.io](https://crates.io/crates/generic-api-client/0.2.1)
- [docs.rs](https://docs.rs/generic-api-client/0.2.1)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/9a2e1ac25587c3981e4348538ba99f3ed93b9817/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/c8019150ed2c0971a50b6c5f0f5fdf2aca254550...9a2e1ac25587c3981e4348538ba99f3ed93b9817)


### Added
- Added static `http::USER_AGENT`. This is the User-Agent string used for all HTTP requests.

### Changed
- All HTTP requests made using this library will include the User-Agent header.

## 0.2.0 (2023-02-28)
- [crates.io](https://crates.io/crates/generic-api-client/0.2.0)
- [docs.rs](https://docs.rs/generic-api-client/0.2.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/c8019150ed2c0971a50b6c5f0f5fdf2aca254550/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/e39e98f5b52cb306ef4a97f0b2675bf48aae8990...c8019150ed2c0971a50b6c5f0f5fdf2aca254550)

### Added
- Added cargo features `native-tls`, `native-tls-vendored`, `rustls-tls-native-roots`, and `rustls-tls-webpki-roots`
to allow users to choose which library to use for TLS connection. None of them is enabled by default.

### Changed
- (BREAKING) `tokio-tungstenite/native-tls` is not enabled by default. In order to connect to `wss://` endpoints,
users have to enable one of the newly added cargo features.

## 0.1.4 (2023-02-05)
- [crates.io](https://crates.io/crates/generic-api-client/0.1.4)
- [docs.rs](https://docs.rs/generic-api-client/0.1.4)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/e39e98f5b52cb306ef4a97f0b2675bf48aae8990/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/980b9f4fe8f6fa128ad1a00bad1e0778b00f9ac4...e39e98f5b52cb306ef4a97f0b2675bf48aae8990)

### Changed
- Changed the signature of `websocket::WebSocketConnection.send_message()` so that it takes `&self` instead of `&mut self`

## 0.1.3 (2023-01-30)
- [crates.io](https://crates.io/crates/generic-api-client/0.1.3)
- [docs.rs](https://docs.rs/generic-api-client/0.1.3)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/980b9f4fe8f6fa128ad1a00bad1e0778b00f9ac4/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/71da3426a58beb064fb1553e8241f8a6e9b25849...980b9f4fe8f6fa128ad1a00bad1e0778b00f9ac4)

This release only includes changes to README.md and dependency updates.

## 0.1.2 (2023-01-05)
- [crates.io](https://crates.io/crates/generic-api-client/0.1.2)
- [docs.rs](https://docs.rs/generic-api-client/0.1.2)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/71da3426a58beb064fb1553e8241f8a6e9b25849/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/5f627177743aa7a48e41aca67989a816710f7856...71da3426a58beb064fb1553e8241f8a6e9b25849)

### Added
- Added re-export of `serde` as `http::serde`
- Added re-export of `tungstenite::Error` as `websocket::TungsteniteError`

### Changed
- Changed the log message emitted when a request timeouts so that it includes the attempt count

## 0.1.1 (2022-12-24)
- [crates.io](https://crates.io/crates/generic-api-client/0.1.1)
- [docs.rs](https://docs.rs/generic-api-client/0.1.1)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/5f627177743aa7a48e41aca67989a816710f7856/generic-api-client)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/20f55729109377702a2ec3340a5783773101d099...5f627177743aa7a48e41aca67989a816710f7856)

### Changed
- (BREAKING) Renamed `websocket::WebSocketConfig.reconnection_window` to `reconnection_wait`

## 0.1.0 (2022-12-22)
- [crates.io](https://crates.io/crates/generic-api-client/0.1.0)
- [docs.rs](https://docs.rs/generic-api-client/0.1.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/20f55729109377702a2ec3340a5783773101d099/generic-api-client)

First release.
