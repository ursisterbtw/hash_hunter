# Use a more recent version of Rust
FROM rust:1.82 AS builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the application
RUN cargo build --release

# Use Ubuntu as the base image for the final stage
FROM ubuntu:22.04

# Install necessary runtime libraries
RUN apt-get update && \
    apt-get install -y \
    libssl3 \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary and supervisor config
COPY --from=builder /usr/src/app/target/release/hash_hunter /usr/local/bin/hash_hunter
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# Set the working directory
WORKDIR /usr/src/app

# Create directory for persistent storage
RUN mkdir -p /usr/src/app/gen

# Use supervisor to manage the process
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
