FROM rustlang/rust:nightly-alpine AS builder
RUN apk add --no-cache \
    musl-dev \
    gcc \
    libc-dev \
    make
RUN addgroup -S appuser && adduser -S -G appuser appuser
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && \
    chown -R appuser:appuser /usr/src/app
USER appuser

FROM ubuntu:24.04
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    supervisor && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/hash_hunter /usr/local/bin/hash_hunter
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
WORKDIR /usr/src/app
RUN mkdir -p /usr/src/app/gen
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD pgrep supervisord || exit 1
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]