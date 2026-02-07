#!/bin/bash

fuser -k 50051/tcp

cd /home/florin/github/crab-talk || exit

export DOCKER_CONFIG=$HOME/.docker-no-creds

if ! docker image inspect otel/opentelemetry-collector-contrib:latest > /dev/null 2>&1; then
    docker pull otel/opentelemetry-collector-contrib:latest || OTLP_SKIP=true
fi

if [ -z "$OTLP_SKIP" ]; then
    docker run --rm -d -p 4317:4317 --name otel-collector otel/opentelemetry-collector-contrib:latest
fi

cleanup() {
    if [ -z "$OTLP_SKIP" ]; then
        docker stop otel-collector
    fi
    exit
}

trap cleanup SIGINT SIGTERM

cargo build -p crab-talk-server -p crab-talk-client
RUST_LOG=info cargo run -p crab-talk-server &
SERVER_PID=$!

sleep 2

cargo run -p crab-talk-client

wait $SERVER_PID
cleanup
