ARG APP_NAME=init

FROM rust:1.81.0 AS builder
ARG APP_NAME

WORKDIR /app
COPY . /app

RUN touch /app/.env \
    && chmod +x /app/.env \
    && chmod +x /app/init.sh \
    && cargo build --release --bin $APP_NAME

ENTRYPOINT ["/app/init.sh"]
