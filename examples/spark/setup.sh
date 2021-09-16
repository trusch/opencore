#!/bin/bash

BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/login_admin.sh
source ${BASEDIR}/../common/functions.sh

echo "Create schema 'datasource'"
schema=$((cat - | jq -c) <<EOF
{
    "title": "datasource",
    "type": "object",
    "properties": {
        "name": {
            "type":"string"
        },
        "url": {
            "type":"string"
        },
        "properties": {
            "type": "object"
        }
    },
    "required": [
        "name",
        "url",
        "properties"
    ],
    "additionalProperties": false
}
EOF
)

payload=$(jq -n --arg schema "$schema" '{kind: "datasource", data: $schema}')
call_opencore catalog.Schemas/Create "$payload" | jq .

echo "Create schema 'etl-job'"
schema=$((cat - | jq -c) <<EOF
{
    "title": "etl-job",
    "type": "object",
    "properties": {
        "name": {
            "type":"string"
        },
        "sql": {
            "type":"string"
        },
        "target": {
            "type": "string"
        },
        "state": {
            "type":"string",
            "default": "PENDING",
            "enum": [
                "PENDING",
                "RUNNING",
                "FINISHED",
                "FAILED"
            ]
        },
        "error": {
            "type": "string"
        }
    },
    "required": [
        "state",
        "sql"
    ],
    "additionalProperties": false
}
EOF
)

payload=$(jq -n --arg schema "$schema" '{kind: "etl-job", data: $schema}')
call_opencore catalog.Schemas/Create "$payload" | jq .

echo "Main: create service account 'data'"
payload=$(jq -n '{name: "data"}')
export DATA_SA_KEY=$(call_opencore idp.ServiceAccounts/Create "$payload" | jq -r .secretKey)
echo $DATA_SA_KEY

echo "Main: create datasource 'postgres'"
data=$(jq -n '{name: "postgres", "url": "jdbc:postgresql://localhost:5432/postgres", properties: {user: "postgres", password: "postgres", driver: "org.postgresql.Driver"}}')
payload=$(jq -n --arg data "$data" '{kind: "datasource", data: $data, shares: [{principal_id: "data", actions: ["read", "write"]}]}')
ds=$(call_opencore catalog.Resources/Create "$payload")
ds_id=$(echo $ds | jq -r .id)
