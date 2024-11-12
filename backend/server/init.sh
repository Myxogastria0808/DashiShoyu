#!/bin/bash

source /app/.env
cargo run --manifest-path /app/migration/Cargo.toml -- refresh -u postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@postgres:$POSTGRES_PORT/$POSTGRES_DB
/app/target/release/init $CSV_DIR_PATH
