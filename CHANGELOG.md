# Changelog

## Unreleased
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/c8019150ed2c0971a50b6c5f0f5fdf2aca254550...main)

### Added
- Module `crypto-botters::exchanges` was added. ([#34](https://github.com/negi-grass/crypto-botters/pull/34))
- Module `crypto-botters::traits` was added. ([#34](https://github.com/negi-grass/crypto-botters/pull/34))

### Changed
- (BREAKING) Type parameters of `WebSocketHandler`s were removed. ([#38](https://github.com/negi-grass/crypto-botters/pull/38))
- (BREAKING) The variants of `crypto-botters::exchanges::bybit::BybitHttpAuth` were changed support the V5 API. ([#35](https://github.com/negi-grass/crypto-botters/pull/35))

### Removed
- Crates `crypto-botters-binance`, `crypto-botters-bitflyer`, `crypto-botters-bybit`, and `crypto-botters-coincheck` were
removed and their functionality are now available in `crypto-botters::exchanges`. ([#34](https://github.com/negi-grass/crypto-botters/pull/34))
- Crate `crypto-botters-api` was removed. Its functionality is now available in `crypto-botters::traits` ([#34](https://github.com/negi-grass/crypto-botters/pull/34))

### Fixed
- Fixed a bug which had been causing authentication to fail for Bybit's older endpoints.

## 0.4.3 (2023-02-28)
- [crates.io](https://crates.io/crates/crypto-botters/0.4.3)
- [docs.rs](https://docs.rs/crypto-botters/0.4.3)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.4.3)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/c8019150ed2c0971a50b6c5f0f5fdf2aca254550)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/880c0acba659e1354e97de3e45992cd2a03d6ba9...c8019150ed2c0971a50b6c5f0f5fdf2aca254550)

### Added
- Added cargo features to allow users to choose which library to use for TLS connection.
The features are `native-tls`, `native-tls-vendored`, `rustls-tls-native-roots`, `rustls-tls-webpki-roots`.
`native-tls` is enabled by default.

## 0.4.2 (2023-01-30)
- [crates.io](https://crates.io/crates/crypto-botters/0.4.2)
- [docs.rs](https://docs.rs/crypto-botters/0.4.2)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.4.2)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/880c0acba659e1354e97de3e45992cd2a03d6ba9)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/e0aa18f1397bb1988d9a9df056d47a06dc0ba997...880c0acba659e1354e97de3e45992cd2a03d6ba9)

### Added
- Added `crypto-botters-binance::BinanceRequestError`
- Added `crypto-botters-bitflyer::BitFlyerRequestError`
- Added `crypto-botters-bybit::BybitRequestError`
- Added `crypto-botters-coincheck::CoincheckRequestError`
- Added `crypto-botters-binance::BinanceOption::Spot4` to support endpoint `https://api4.binance.com`
- Added implementation of `Clone` for `crypto-botters::Client`

## 0.4.1 (2023-01-23)
- [crates.io](https://crates.io/crates/crypto-botters/0.4.1)
- [docs.rs](https://docs.rs/crypto-botters/0.4.1)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.4.1)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/e0aa18f1397bb1988d9a9df056d47a06dc0ba997)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/86757b297da3912eea165222c271653b08fd3947...e0aa18f1397bb1988d9a9df056d47a06dc0ba997)

This release fixes a mistake in README.md

## 0.4.0 (2023-01-23) (YANKED)
- [crates.io](https://crates.io/crates/crypto-botters/0.4.0)
- [docs.rs](https://docs.rs/crypto-botters/0.4.0)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.4.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/86757b297da3912eea165222c271653b08fd3947)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/e00e34cdf04d01ba5fada14345fa43f2d4d989d4...86757b297da3912eea165222c271653b08fd3947)

### Added
- Support for Bybit WebSocket API was added. ([#10](https://github.com/negi-grass/crypto-botters/pull/10))
- Support for Coincheck API was added. ([#11](https://github.com/negi-grass/crypto-botters/pull/11))

## 0.3.0 (2023-01-05)
- [crates.io](https://crates.io/crates/crypto-botters/0.3.0)
- [docs.rs](https://docs.rs/crypto-botters/0.3.0)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.3.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/e00e34cdf04d01ba5fada14345fa43f2d4d989d4)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/030f7757dce979a8f19cb0a5c8744ea06e124e83...e00e34cdf04d01ba5fada14345fa43f2d4d989d4)

This release drastically changes how users use this library. ([#6](https://github.com/negi-grass/crypto-botters/pull/6))

I will not list all changes as there are too many of them.

### Added
- Added `crypto-botters::Client` so that users don't have to deal with `RequestHandler`s directly.
- Many `struct`s, `enum`s and `trait`s were added.
- Support for Bybit HTTP API was added. ([#8](https://github.com/negi-grass/crypto-botters/pull/8))

### Changed
- (BREAKING) Users will use `crypto-botters::Client` from now on.

### Removed
- (BREAKING) Many `struct`s were removed.

## 0.2.0 (2022-12-26)
- [crates.io](https://crates.io/crates/crypto-botters/0.2.0)
- [docs.rs](https://docs.rs/crypto-botters/0.2.0)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.2.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/030f7757dce979a8f19cb0a5c8744ea06e124e83)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/5f627177743aa7a48e41aca67989a816710f7856...030f7757dce979a8f19cb0a5c8744ea06e124e83)

### Added
- Added `crypto-botters-binance::BinanceHttpUrl::EuropeanOptions` to support endpoint `https://eapi.binance.com` ([#2](https://github.com/negi-grass/crypto-botters/issues/2))
- Added `crypto-botters-binance::BinanceWebSocketUrl::EuropeanOptions` to support endpoint `wss://nbstream.binance.com` ([#2](https://github.com/negi-grass/crypto-botters/issues/2))

### Changed
- (BREAKING) Renamed `crypto-botters-binance::RequestResult` to `BinanceRequestResult`
- (BREAKING) Renamed `crypto-botters-bitlyer::RequestResult` to `BitFlyerRequestResult`

### Fixed
- Fixed an issue which had been preventing `RequestHandler`s from being `Send` ([#3](https://github.com/negi-grass/crypto-botters/issues/3))

## 0.1.1 (2022-12-24) (YANKED)
- [crates.io](https://crates.io/crates/crypto-botters/0.1.1)
- [docs.rs](https://docs.rs/crypto-botters/0.1.1)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/5f627177743aa7a48e41aca67989a816710f7856)
- [full diff on GitHub](https://github.com/negi-grass/crypto-botters/compare/163d10ce850586140c9b29585a04c962617a49a4...5f627177743aa7a48e41aca67989a816710f7856)

### Added
- Added `crypto-botters-binance::Binance.request_max_try`, `crypto-botters-bitflyer::BitFlyer.request_max_try`

## 0.1.0 (2022-12-22) (YANKED)
- [crates.io](https://crates.io/crates/crypto-botters/0.1.0)
- [docs.rs](https://docs.rs/crypto-botters/0.1.0)
- [GitHub release](https://github.com/negi-grass/crypto-botters/releases/tag/v0.1.0)
- [snapshot on GitHub](https://github.com/negi-grass/crypto-botters/tree/163d10ce850586140c9b29585a04c962617a49a4)

First release.
