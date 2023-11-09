#!/bin/bash
source .env
curl -H "Content-Type: application/json" -d '{"id":"1", "jsonrpc":"2.0", "method": "state_getMetadata", "params":[]}' $HTTP_ENDPOINT > curio-parachain.json