#!/usr/bin/env bash
set -euo pipefail

# Settings
MONGO_VERSION=7
CONTAINER_NAME=test-mongo
MONGO_PORT=27017

# Start MongoDB container
echo "Starting MongoDB container..."
docker run -d --name $CONTAINER_NAME -p $MONGO_PORT:27017  mongo:$MONGO_VERSION


# Define cleanup function
cleanup() {
    echo "Stopping Mongo container..."
    docker stop "$CONTAINER_NAME" >/dev/null
    docker rm "$CONTAINER_NAME" >/dev/null
    echo "Mongo container stopped and removed."
}

# Trap script exit and run cleanup
trap cleanup EXIT

# Wait for MongoDB to be ready
echo "Waiting for MongoDB to be ready..."
until docker exec "$CONTAINER_NAME" mongosh --eval "db.adminCommand('ping')" >/dev/null 2>&1; do
  sleep 1
done

# Export MONGO_URI
export MONGO_URI="mongodb://localhost:$MONGO_PORT"

# Run tests with cargo nextest
echo "Running tests with cargo nextest..."
cargo nextest run "$@"
