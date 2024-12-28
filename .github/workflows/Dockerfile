FROM rust:nightly AS builder

RUN groupadd -r appuser && useradd -r -g appuser appuser
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY . .

CMD ["cargo build"]
