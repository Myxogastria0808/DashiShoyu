#!/bin/bash

echo "Sleep 1s"
sleep 1s
echo $POSTGRES_USER
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_CONTAINER_NAME:$POSTGRES_PORT/$POSTGRES_DB
MEILISEARCH_API_KEY_RESULT=`curl -X GET "$MEILI_DOCKER_URL/keys" -H "Authorization: Bearer $MASTER_KEY" | jq -r '.results[1].key' | sed -n '$p'`
export ADMIN_API_KEY=$MEILISEARCH_API_KEY_RESULT
cargo run --bin init $CSV_DIR_PATH
