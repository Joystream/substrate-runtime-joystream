#!/usr/bin/env bash


# Must be run on the clean development chain.
# It tests all commands.

# Set Alice as leader
yarn storage-node dev:init # leader workerId = 0

# Update limits and constants
yarn storage-node leader:update-bag-limit -l 7 --dev
yarn storage-node leader:update-voucher-limits -o 100 -s 10000000 --dev
yarn storage-node leader:update-data-fee -f 10000 --dev

# Create and configure a bucket.
yarn storage-node leader:create-bucket -i=0 --dev # bucketId = 0
yarn storage-node operator:accept-invitation -w=0 -i=0 --dev
yarn storage-node operator:set-bucket-limits -w=0 -i=0 -o=100 -s=10000000 --dev
yarn storage-node operator:update-bucket-status -w=0 -i=0 --enable --dev
yarn storage-node leader:update-bag -b=0 -i static:council --dev 
yarn storage-node operator:set-metadata -w=0 -i=0 -m=http://google.com --dev

# Create and delete a bucket
yarn storage-node leader:create-bucket -a -n=100 -s=10000000  --dev # bucketId = 1
yarn storage-node leader:delete-bucket -i=1  --dev 

# Create a bucket, invite operator, cancel invite, accept invitation and remove operator.
yarn storage-node leader:create-bucket -a -n=100 -s=10000000  --dev # bucketId = 2
yarn storage-node leader:invite-operator -i=2 -w=0  --dev 
yarn storage-node leader:cancel-invite -i=2   --dev 
yarn storage-node leader:invite-operator -i=2 -w=0  --dev 
yarn storage-node operator:accept-invitation -w=0 -i=2 --dev
yarn storage-node leader:remove-operator -i=2   --dev 

# Toggle uploading block.
yarn storage-node leader:set-uploading-block --enable --dev 
yarn storage-node leader:set-uploading-block --disable --dev 