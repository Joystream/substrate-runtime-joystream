# Substrate ingest node

This tool is responsible for taking quering the blockchain data and posting to the relevant ElasticSearch indexes. It will create ES indices `events` and `members` and populate them from the Joystream blockchain.

## Install

To compile and install dependencies, run

```[bash]
cd substrate-ingest && yarn install
```

To run, first make sure the source joystream node and sink elasticsearch node are up and running. Inspect `config.yml` and verify endpoints. By default, it points to a one-node local cluster of ElasticSearch which can be run from Docker together with a Kibana dashboard server (optional):

```[bash]
docker-compose up
```

## Run

Inspect `substrate-ingest/config.yml` and type

```[bash]
yarn start
```

It will create ES indices `events` and `members` and populate them from the Joystream blockchain. One can inspect the indices with `curl` or Kibana:

```[bash]
curl http://localhost:9200/members/_search?pretty
curl http://localhost:9200/events/_search?pretty
```
