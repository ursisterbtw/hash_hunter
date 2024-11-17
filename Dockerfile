FROM rust:1.82 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
WORKDIR /usr/src/app

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    && rm -rf /var/lib/apt/lists/*

# Create necessary directories and copy config
COPY src/config.yml /usr/src/app/config.yml

# Copy the binary
COPY --from=builder /usr/src/app/target/release/hash_hunter /usr/local/bin/

# Create gen directory with proper permissions
RUN mkdir -p gen && \
    chmod 777 gen

# Set the entrypoint
ENTRYPOINT ["hash_hunter"]