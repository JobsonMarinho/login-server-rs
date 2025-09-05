#!/usr/bin/env bash
set -euo pipefail

URL=${1:-http://localhost:8080/login}
RATE=${RATE:-1000}    # requests per second
DUR=${DUR:-30s}
WORKERS=${WORKERS:-4}

echo "Benchmarking HTTP ${URL} at ${RATE} rps for ${DUR} ..."

# requires vegeta installed locally; fallback to docker image
if command -v vegeta >/dev/null 2>&1; then
  echo "Using local vegeta"
  echo "POST ${URL}
Content-Type: application/json

{\"account\":\"test\",\"password\":\"test\"}" | vegeta attack -duration=$DUR -rate=$RATE -workers=$WORKERS | tee results.bin | vegeta report
  vegeta plot results.bin > http-plot.html
  echo "Wrote http-plot.html"
else
  echo "Using dockerized vegeta"
  echo -e "POST ${URL}\nContent-Type: application/json\n\n{\"account\":\"test\",\"password\":\"test\"}" \
  | docker run --rm -i peterevans/vegeta sh -lc "vegeta attack -duration=$DUR -rate=$RATE -workers=$WORKERS | tee results.bin | vegeta report && vegeta plot results.bin" > http-plot.html
  echo "Wrote http-plot.html"
fi
