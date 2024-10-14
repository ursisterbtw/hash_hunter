# hash_hunter

Hash Hunter is a simple Ethereum vanity address generator written in Rust. It is designed to be a proof of concept for generating Ethereum addresses with a specific prefix and suffix. The project is a work in progress and is not intended for production use.

Preliminary setup finished, python/rust working but cython not fully integrated yet. This project is a work in progress, don't ever use use the keys produced by it. Cuidado loco!

## Usage

```wsl
cargo run -- -p 6969 -e 6969 -c -s 100000 -m 10000000000 -i 10000
```
