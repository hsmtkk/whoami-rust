FROM rust:1.53 AS builder

WORKDIR /opt

COPY src /opt/src
COPY Cargo.lock /opt/Cargo.lock
COPY Cargo.toml /opt/Cargo.toml

RUN cargo install --path .

FROM ubuntu:20.04

COPY --from=builder /usr/local/cargo/bin/whoami-rust /usr/local/bin/whoami-rust

ENTRYPOINT ["/usr/local/bin/whoami-rust"]
