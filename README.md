# hash_hunter

hash_hunter is a simple Ethereum vanity address generator written in both Rust and Python. It is designed to be a proof of concept for generating Ethereum addresses with a specific prefix and suffix, as well as some other features.

🦀 main.rs 🦀 is designed to max out cpu, 🐍 main.py 🐍 is a little more considerate.

Setup finished, python/rust working as intended.

If you're feeling froggy, I left some hints in /src that point towards a rather speedy Cython implementation ⏩

Cuidado loco!

## Usage

```rust
```rust
cargo run --release
```

```rust
```rust
cargo run --release -- --start-pattern 123 --end-pattern abc --min-zeros 5
```

## **THIS PROJECT IS A WORK IN PROGRESS, DON'T EVER USE THE KEYS PRODUCED BY IT IN PRODUCTION OR ON MAINNET**
