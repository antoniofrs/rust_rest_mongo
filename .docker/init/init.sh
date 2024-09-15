#!/usr/bin/env bash

echo "configuring sqs"
echo "==================="
LOCALSTACK_HOST=localhost
AWS_REGION=eu-central-1

create_queue() {
    local QUEUE_NAME_TO_CREATE=$1
    awslocal --endpoint-url=http://${LOCALSTACK_HOST}:4566 sqs create-queue --queue-name "${QUEUE_NAME_TO_CREATE}" --region ${AWS_REGION} --attributes VisibilityTimeout=30
}

create_queue "queue_1"
create_queue "queue_2"
create_queue "queue_3"
create_queue "queue_4"

awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_1 --message-body "1-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_1 --message-body "2-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_2 --message-body "1-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_2 --message-body "2-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_3 --message-body "1-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_3 --message-body "2-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_4 --message-body "1-test"
awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/queue_4 --message-body "2-test"