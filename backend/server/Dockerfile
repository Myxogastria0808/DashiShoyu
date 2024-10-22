ARG APP_NAME=server

FROM rust:1.81.0 AS builder
ARG APP_NAME

WORKDIR /app
COPY . /app

RUN cargo build --release --bin $APP_NAME

EXPOSE 5000

ENTRYPOINT ["/app/target/release/server"]
