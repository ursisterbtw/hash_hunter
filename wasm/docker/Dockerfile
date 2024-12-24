FROM rust:1.85-slim

WORKDIR /app

# Copy the entire project
COPY . .

# Build the project
RUN cargo build --release

# Run the binary
ENTRYPOINT ["./target/release/hash_hunter"]