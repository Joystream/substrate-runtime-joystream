#!/usr/bin/env bash
set -e

SCRIPT_PATH="$(dirname "${BASH_SOURCE[0]}")"
cd $SCRIPT_PATH

set -a
. ../.env
set +a

# Only remove query-node related services
docker-compose rm -vsf processor-mnt
docker-compose rm -vsf graphql-server-mnt
docker-compose rm -vsf indexer
docker-compose rm -vsf hydra-indexer-gateway
docker-compose rm -vsf redis
docker-compose rm -vsf db
