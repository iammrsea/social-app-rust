#!/bin/bash

# Wait for MongoDB to be available
until mongosh --host mongodb --port 27017 --eval "db.adminCommand('ping')" >/dev/null 2>&1; do
  echo "⏳ Waiting for MongoDB to start..."
  sleep 2
done

echo "✅ MongoDB is up. Initiating replica set..."

mongosh --host mongodb --port 27017 <<EOF
rs.initiate({
  _id: "rs0",
  members: [{ _id: 0, host: "host.docker.internal:27017" }]
});
EOF

echo "✅ Replica set initiated."
