A minimal rewrite of SoftEtherVPN's Mayaqua "kernel" at the heart of the VPN.

Uses minimal external dependencies: well known, maintained crates: tokio, socket2, openssl

TODO: Check panic behaviour:

```
rustup component add miri
cargo miri setup
cargo miri test
```

```
cargo clippy -- -W clippy::panic
cargo clippy -- -W clippy::indexing_slicing
```