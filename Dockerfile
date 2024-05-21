FROM rust:latest AS chef

WORKDIR /mpesa_sdk

RUN cargo install cargo-chef --locked

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /mpesa_sdk/recipe.json recipe.json

# Caching Docker layer.
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

CMD ["./target/release/mpesa-sdk"]