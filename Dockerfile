FROM rust:1.82 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    supervisor && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/hash_hunter /usr/local/bin/hash_hunter
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
WORKDIR /usr/src/app
RUN mkdir -p /usr/src/app/gen
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]