FROM rust:latest as nextest
RUN cargo install cargo-nextest --locked

FROM nextest AS builder
WORKDIR /build/src
COPY . /build/src/