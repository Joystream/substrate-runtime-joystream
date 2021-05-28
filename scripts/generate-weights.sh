#!/usr/bin/env bash

# Executes and replaces all benchmarks with the new weights

SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}")

STEPS=${1:-50}
REPEAT=${2:-20}

benchmark() {
  echo "Generating weights for $1..."
  start=`date +%s`
  ERROR=$($SCRIPT_DIR/../target/release/joystream-node benchmark \
      --pallet=$1 \
      --extrinsic=* \
      --chain=dev \
      --steps=$STEPS \
      --repeat=$REPEAT \
      --execution=wasm \
      --output=. 2>&1 > /dev/null)


  if [[ $ERROR != *"Error"* ]]; then
      end=`date +%s`
      mv $SCRIPT_DIR/../*.rs $SCRIPT_DIR/../runtime/src/weights/
      echo "Weights generated successfully for $1"
      echo "It took $((end-start)) seconds"
  else
      >&2 echo "$ERROR"
      >&2 echo "There was a problem generating the weights for $1, check the error above"
      exit 1
  fi
}

# Joystream benchmarks
benchmark storage_v2
