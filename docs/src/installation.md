# Installation

In order to use derive macros provided by [**edgedb-query**]() crate you need to add several crates in your _cargo.toml_ file.

```toml
[dependencies]
edgedb-query = { path = "../edgedb-query" }
edgedb-query-derive = { path = "../edgedb-query-derive" }
```

You also need to add [edgedb-rust](https://github.com/edgedb/edgedb-rust) crates 👇

```toml
edgedb = "0.1"
edgedb-tokio = "0.2"
edgedb-protocol = "0.3"
```
Since you are going to use edgedb-tokio, you will also need to add [tokio](https://github.com/tokio-rs/tokio) crate 👇 

```toml
tokio = { version = "1.19.2", features = ["full"] }
```
