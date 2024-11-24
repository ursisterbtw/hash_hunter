# ☁️ hash_hunter ☁️

[!saturdaynight](./assets/saturdaynight.gif)

`hash_hunter` is an Ethereum vanity address generator written in both Rust and Python. It is designed to be a proof of concept for generating Ethereum addresses with a specific prefix and suffix, as well as some other features.

## Features

- Multi-language implementation (Rust, Python, Cython)
- Pattern matching for address prefixes and suffixes
- EIP-55 checksum support
- Rarity scoring system
- Multi-threaded processing
- Progress tracking and logging
- Docker support
- Configurable parameters
- Address verification + entropy scoring

🦀 `main.rs` 🦀 is designed to max out cpu, 🐍 `main.py` 🐍 is a little more considerate.

If you're feeling froggy, I left some hints in /src that point towards a rather speedy Cython implementation ⏩

Be careful!

## Usage

```rust
cargo run --release
```

```rust
cargo run --release -- --start-pattern 123 --end-pattern abc --min-zeros 5
```

## **THIS PROJECT IS A WORK IN PROGRESS, DON'T EVER USE THE KEYS PRODUCED BY IT IN PRODUCTION OR ON MAINNET**
