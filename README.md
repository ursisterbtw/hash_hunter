<h1 align="center">☁️ hash_hunter ☁️</h1>

<p align="center">

  <img src="./saturdaynight.gif" alt="saturdaynight" width="300" />

</p>

`hash_hunter` is an Ethereum vanity address generator written in both Rust and Python. It is designed to be a proof of concept for generating Ethereum addresses with a specific prefix and suffix, as well as other patterns like palindromes, ascending/descending sequences, and hexspeak.

## Features

- **Rust Implementation (`main.rs`)**:

  - Maximize CPU utilization for faster address generation.
  - Supports customizable patterns including prefix, suffix, and regex patterns.
  - Provides options for enabling EIP-55 checksum and skipping confirmation prompts.
  - Displays entropy estimation and years to crack for generated addresses.
  - Saves wallet information to files upon finding a match.

- **Python Implementation (`main.py`)**:
  - Utilizes multithreading for concurrent address generation.
  - Matches addresses against predefined patterns (e.g., four zeros, triple digits, ascending/descending sequences).
  - Calculates rarity scores for generated addresses.
  - Saves wallet information to files upon finding a match.

## Usage

### Rust

To run the Rust implementation:

```rust
cargo run --release
```

With additional parameters:

```rust
cargo run --release -- --start-pattern 123 --end-pattern abc --min-zeros 5
```

### Python

To run the Python implementation:

```python
python main.py
```

With additional parameters:

```python
python main.py --start-pattern 123 --end-pattern abc --checksum
```

## Contributions

Feel free to contribute by submitting issues or pull requests. Be careful when testing and ensure you do not use the generated keys on the main Ethereum network.

## License

This project is licensed under the MIT License.
