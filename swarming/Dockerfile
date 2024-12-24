FROM rustlang/rust:nightly-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/hash_hunter .
CMD ["./hash_hunter", "-p", "000000000000", "-c", "-s", "100000", "-m", "100000000", "-i", "10000", "-y"]
