#!/bin/bash

source ./.env
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$POSTGRES_PORT/$POSTGRES_DB
cargo run --bin init $CSV_DIR_PATH
