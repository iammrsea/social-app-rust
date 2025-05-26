#!/usr/bin/env bash
set -euo pipefail

# Settings
COMPOSE_FILE="docker-compose-test.yml"
MONGO_PORT=27017

# Start MongoDB container
echo "Starting MongoDB container..."
docker-compose -f "$COMPOSE_FILE" up -d
echo "MongoDB container started."

# Define cleanup function
cleanup() {
    echo "Stopping Mongo container..."
    # docker stop "$CONTAINER_NAME" >/dev/null
    # docker rm "$CONTAINER_NAME" >/dev/null
    docker-compose -f "$COMPOSE_FILE" down
    echo "Mongo container stopped and removed."
}

# Trap script exit and run cleanup
trap cleanup EXIT

# Wait for mongo-init to finish (replica set ready)
echo "Waiting for replica set initialization to complete..."
until [ "$(docker inspect -f '{{.State.Status}}' test-mongo-init 2>/dev/null)" = "exited" ]; do
  sleep 1
done

# Export MONGO_URI
export MONGO_URI="mongodb://localhost:$MONGO_PORT/?replicaSet=rs0"

# Run tests with cargo nextest
echo "Running tests with cargo nextest..."
cargo nextest run "$@"
