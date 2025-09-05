#!/usr/bin/env bash
set -euo pipefail

ADDR=${1:-localhost:50051}
RATE=${RATE:-1000}
DUR=${DUR:-30s}
CPUS=${CPUS:-4}

# We'll use ghz (https://github.com/bojand/ghz). If not installed, use docker image.
cat > /tmp/login.json <<JSON
{
  "account": "test",
  "password": "test",
  "ip": "127.0.0.1"
}
JSON

if command -v ghz >/dev/null 2>&1; then
  echo "Using local ghz"
  ghz --insecure --proto ./proto/login.proto --call login.Login/Login \
      -c $CPUS -z $DUR -q $RATE -d @/tmp/login.json $ADDR
else
  echo "Using dockerized ghz"
  docker run --rm -v "$PWD:/data" ghzio/ghz \
      --insecure --proto /data/proto/login.proto --call login.Login/Login \
      -c $CPUS -z $DUR -q $RATE -d @/data/scripts/login.json \
      $ADDR
fi
