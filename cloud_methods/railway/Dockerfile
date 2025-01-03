FROM rustlang/rust:nightly-alpine AS builder
RUN apk add --no-cache \
    musl-dev \
    gcc \
    libc-dev \
    make
WORKDIR /cloud_methods/railway
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /cloud_methods/railway/target/release/hash_hunter /usr/local/bin/hash_hunter
WORKDIR /hash_hunter
RUN mkdir -p /hash_hunter/gen
ENV PORT=6969
EXPOSE 6969
CMD ["/usr/local/bin/hash_hunter", "-p", "000000000000", "-c", "-s", "100000", "-m", "100000000000", "-i", "10000", "-y"]
