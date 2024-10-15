#!/bin/bash

docker-compose up -d

echo "Sleep 1s"
sleep 1s

. ./.env
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$POSTGRES_PORT/$POSTGRES_DB
MEILISEARCH_API_KEY_RESULT=`curl -X GET "$MEILI_URL/keys" -H "Authorization: Bearer $MASTER_KEY" | jq -r '.results[1].key' | sed -n '$p'`
MEILISEARCH_API_KEY="ADMIN_API_KEY=$MEILISEARCH_API_KEY_RESULT"
echo "`sed "s/ADMIN_API_KEY=/$MEILISEARCH_API_KEY/" ./.env | sed 's/ /\n/g'`" > ./.env
cargo run --bin init $CSV_DIR_PATH
