#!/bin/bash

set -e

if [ -z "$1" ]; then
  echo "ERROR: Configuration file not passed"
  echo "Please use ./delete-stack.sh PATH/TO/CONFIG to run this script"
  exit 1
else
  echo "Using $1 file for config"
  source $1
fi

get_aws_export () {
  RESULT=$(aws cloudformation list-exports \
    --profile $CLI_PROFILE \
    --query "Exports[?starts_with(Name,'${NEW_STACK_NAME}$1')].Value" \
    --output text | sed 's/\t\t*/\n/g')
  echo -e $RESULT | tr " " "\n"
}

BUCKET_NAME=$(get_aws_export "S3BucketName")

# Delete the CloudFormation stack

echo -e "\n\n=========== Emptying bucket $BUCKET_NAME ==========="

aws s3 rm s3://$BUCKET_NAME --recursive --profile $CLI_PROFILE

echo -e "\n\n=========== Deleting stack $NEW_STACK_NAME ==========="

aws cloudformation delete-stack --stack-name $NEW_STACK_NAME --profile $CLI_PROFILE

echo -e "\n\n=========== Waiting for stack deletion to complete ==========="

aws cloudformation wait stack-delete-complete --stack-name $NEW_STACK_NAME --profile $CLI_PROFILE

echo -e "\n\n=========== Stack $NEW_STACK_NAME deleted ==========="
