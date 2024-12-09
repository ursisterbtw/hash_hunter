FROM rust:1.82 AS builder
RUN groupadd -r appuser && useradd -r -g appuser appuser
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && \
    chown -R appuser:appuser /usr/src/app
USER appuser

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3=3.0.2-0ubuntu1.15 \
    supervisor=4.2.1-2ubuntu4 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/hash_hunter /usr/local/bin/hash_hunter
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
WORKDIR /usr/src/app
RUN mkdir -p /usr/src/app/gen
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD pgrep supervisord || exit 1
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]