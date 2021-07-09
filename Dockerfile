FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as planner

WORKDIR /opt

COPY src /opt/src
COPY Cargo.lock /opt/Cargo.lock
COPY Cargo.toml /opt/Cargo.toml

RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as cacher

WORKDIR /opt

COPY --from=planner /opt/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53.0 as builder

WORKDIR /opt

COPY src /opt/src
COPY Cargo.lock /opt/Cargo.lock
COPY Cargo.toml /opt/Cargo.toml

COPY --from=cacher /opt/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo install --path .

FROM ubuntu:20.04 as runtime

COPY --from=builder /usr/local/cargo/bin/whoami-rust /usr/local/bin/whoami-rust

ENTRYPOINT ["/usr/local/bin/whoami-rust"]
