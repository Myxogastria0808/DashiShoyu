#!/bin/bash

docker-compose down
sudo rm -rf db meili_data neo4j
sudo rm .env
