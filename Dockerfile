FROM rust:1.66-slim-buster as build

WORKDIR /root

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build

CMD /root/target/debug/axum-sandbox
