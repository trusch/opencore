#!/bin/bash

BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/functions.sh

eval $(cat .list-creds)
payload=$(jq -n --arg KEY "$OPENCORE_ADMIN_KEY" '{service_account_id: "root", password: $KEY}')
export TOKEN=$(grpcurl --plaintext -d "$payload" localhost:3001 idp.Authentication/Login | jq -r .accessToken)

echo "List and count all new resources"
time (call_opencore catalog.Resources/List '{"kind": "animal"}' | jq -c | wc -l)

echo "List and count all new resources with a specific filter $.age > 90 && $.age <= 95"
time (call_opencore catalog.Resources/List '{"kind": "animal", "filter": "$.age > 90 && $.age <= 95"}' | jq -c | wc -l)

echo "List and count all new resources with a specific filter $.name == \"surprising_tom\""
time (call_opencore catalog.Resources/List '{"kind": "animal", "filter": "$.name == \"surprising_tom\""}' | jq -c | wc -l)

echo "List and count all new resources with a specific filter $.name like_regex '^angry'"
time (call_opencore catalog.Resources/List '{"kind": "animal", "filter": "$.name like_regex \"^angry\""}' | jq -c | wc -l)

echo "List and count all new resources with a specific query 'tom zebra'"
time (call_opencore catalog.Resources/List '{"kind": "animal", "query": "zebra tom"}' | jq -c | wc -l)
