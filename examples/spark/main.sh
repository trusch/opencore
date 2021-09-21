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
            "type":"string",
            "x-unique": true
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
            "type": "string"
        },
        "sql": {
            "type": "string"
        },
        "target": {
            "type": "string"
        },
        "state": {
            "type": "string",
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
        "sql",
        "target"
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

echo "Main: create datasource 'postgres'"

# define datasource
ds=$(jq -nc '{
    name: "postgres", 
    url: "jdbc:postgresql://localhost:5432/postgres", 
    properties: {
        user: "postgres", 
        password: "postgres", 
        driver: "org.postgresql.Driver"
    }
}')

# define create payload
payload=$(jq -n --arg data "$ds" '{
    kind: "datasource", 
    data: $data, 
    shares: [{
        principal_id: "data", 
        actions: ["read", "write"]
    }]
}')

# create the resource
ds=$(call_opencore catalog.Resources/Create "$payload")
ds_id=$(echo $ds | jq -r .id)

# start worker
podman run -d --pod opencore --name spark-worker containers.trusch.io/opencore/spark/worker:latest \
    spark-submit --jars "/app/jars/*.jar" /app/main.py \
        --opencore_addr=localhost:3001 \
        --service_account_id=data \
        --service_account_token="${DATA_SA_KEY}"

# create etl job
data=$(jq -nc '{
    "name": "test-job", 
    "sql": "SELECT * FROM @postgres.resources", 
    "target":"@postgres.resources_copy",
    "state":"PENDING"
}')
payload=$(jq -n --arg data "$data" '{
    kind: "etl-job", 
    data: $data, 
    shares: [{
        principal_id: "data", 
        actions: ["read", "write"]
    }]
}')

job=$(call_opencore catalog.Resources/Create "$payload")
job_id=$(echo $job | jq -r .id)
echo "Main: new job id: $job_id"

start_ms=$(($(date +%s%N)/1000000)) # timestamp in milliseconds

echo "Main: listen for update events on $job_id"
payload=$(printf '{"resource_id": "%s", "event_type": "UPDATE"}' $job_id)
call_opencore catalog.Events/Subscribe "$payload" | jq -c --unbuffered | while read -r line; do
    state=$(echo $line | jq -r .data | jq -r .state); \
    if [ "$state" == "FINISHED" ]; then
        echo "Main: Got the result:"
        end_ms=$(($(date +%s%N)/1000000)) # timestamp in milliseconds
        echo "Main: waiting for this took $(echo "$end_ms - $start_ms" | bc -l)ms"
        PGPASSWORD=password psql -h localhost -U postgres -c "SELECT resource_id, kind, updated_at FROM resources_copy ORDER BY updated_at ASC"
    else
        echo "Main: ETL job is running now"
    fi
done

