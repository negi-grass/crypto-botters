# Generic API Client

This library is mainly for API client library developers.

Using this crate, you can use the **same** client to interact with **multiple different**
APIs with, different authentication methods, data formats etc.

The handler traits allow you to do things like authentication, data parsing etc.

For a more detailed explanation on what this crate can do, see the documentation
in the source code.

An example of a library that depends on this crate:
[crypto-botters](https://crates.io/crates/crypto-botters)
