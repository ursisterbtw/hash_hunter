<div align="center">
  <h1>☁️ hash_hunter ☁️</h1>
  <img src="./saturdaynight.gif" width="300" alt="saturdaynight">
</div>

`hash_hunter` is an Ethereum vanity address generator written in both Rust and Python. It is designed to be a proof of concept for generating Ethereum addresses with a specific prefix and suffix, as well as other patterns like palindromes, ascending/descending sequences, and hexspeak.

## Features

- **Rust Implementation (`main.rs`)**:
  - Maximize CPU utilization for faster address generation.
  - Supports customizable patterns including prefix, suffix, and regex patterns.
  - Provides options for enabling EIP-55 checksum and skipping confirmation prompts.
  - Displays entropy estimation and years to crack for generated addresses.

- **Python Implementation (`main.py`)**:
  - Utilizes multithreading for concurrent address generation.
  - Matches addresses against predefined patterns (e.g., four zeros, triple digits, ascending/descending sequences).
  - Calculates rarity scores for generated addresses.
  - Saves wallet information to files upon finding a match.

## Usage

### Rust

To run the Rust implementation:

```bash
cargo run --release
