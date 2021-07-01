#!/usr/bin/env bash

# Compute a hash over files related to building joystream/node docker image

# Cargo workspace root
export WORKSPACE_ROOT=`cargo metadata --offline --no-deps --format-version 1 | jq .workspace_root -r`

cd ${WORKSPACE_ROOT}

TAR=tar
if [[ "$OSTYPE" == "darwin"* ]]; then
	TAR=gtar
fi

export TEST_NODE_BLOCKTIME=1000
export TEST_PROPOSALS_PARAMETERS_PATH="./tests/integration-tests/proposal-parameters.json"

# sort/owner/group/mtime arguments only work with gnu version of tar!
${TAR} -c --sort=name --owner=root:0 --group=root:0 --mtime='UTC 2020-01-01' \
    Cargo.lock \
    Cargo.toml \
    runtime \
    runtime-modules \
    utils/chain-spec-builder \
    joystream-node.Dockerfile \
    $(test -n "$TEST_NODE" && echo "$TEST_PROPOSALS_PARAMETERS_PATH") \
    | if [[ -n "$TEST_NODE" ]]; then sed '$a'"$TEST_NODE_BLOCKTIME"; else tee; fi \
    | shasum \
    | cut -d " " -f 1
