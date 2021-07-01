#!/usr/bin/env bash
set -e

SCRIPT_PATH="$(dirname "${BASH_SOURCE[0]}")"
cd $SCRIPT_PATH

# pass the scenario name without .ts extension
SCENARIO=$1
# fallback if scenario if not specified
SCENARIO=${SCENARIO:=full}

if [ -n "$RUNTIME_UPGRADE_TARGET_IMAGE_TAG" ]; then
  export RUNTIME_UPGRADE_TARGET_WASM_PATH=${RUNTIME_UPGRADE_TARGET_WASM_PATH:="./target-runtime.wasm"}
  id=`docker create joystream/node:${RUNTIME_UPGRADE_TARGET_IMAGE_TAG}`
  docker cp $id:/joystream/runtime.compact.wasm $RUNTIME_UPGRADE_TARGET_WASM_PATH
  docker rm $id
fi

# Execute the tests
time DEBUG=* yarn workspace integration-tests node-ts-strict src/scenarios/${SCENARIO}.ts
