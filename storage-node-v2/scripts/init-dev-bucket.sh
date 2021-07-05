#!/usr/bin/env bash

# Must be run on the clean development chain.
# It prepares an environment, creates a storage bucket and links it to the
# 'static council bag'.

yarn storage-node dev:init
yarn storage-node leader:update-bag-limit -l 7 --dev
yarn storage-node leader:update-voucher-limits -o 100 -s 10000000 --dev
yarn storage-node leader:create-bucket -i=0 -a -n=100 -s=10000000  --dev 
yarn storage-node operator:accept-invitation -w=0 -i=0 --dev
yarn storage-node leader:update-bag -b=0 -i static:council --dev 