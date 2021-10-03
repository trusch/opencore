#!/bin/bash

BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/functions.sh
source ${BASEDIR}/../common/login_admin.sh

echo "test fencing"

echo "Create schema 'person'"
schema=$((cat - | jq -c) <<EOF
{
    "title": "Person",
    "type": "object",
    "properties": {
        "name": {
            "type":"string"
        }
    },
    "additionalProperties": false
}
EOF
)
payload=$(jq -n --arg schema "$schema" '{kind: "person", data: $schema}')
schema=$(call_opencore catalog.Schemas/Create "$payload")

call_opencore catalog.Locks/Lock '{"lock_id": "my_lock"}' | jq -c --unbuffered | {
    read line;
    echo $line;
    fencing_token=$(echo $line | jq -r .fencingToken);
    echo "fencing_token: $fencing_token";

    echo "try creating person with correct fencing token"
    grpcurl --plaintext \
        -H "Authorization: Bearer ${TOKEN}" \
        -H "X-Fencing-Token: my_lock#${fencing_token}" \
        -d '{"kind":"person", "data":"{\"name\":\"alice\"}"}'\
        localhost:3001 catalog.Resources/Create

    echo "try creating person with invalid fencing token"
    let "wrong_fencing_token = ${fencing_token} - 1"
    grpcurl --plaintext \
        -H "Authorization: Bearer ${TOKEN}" \
        -H "X-Fencing-Token: my_lock#${wrong_fencing_token}" \
        -d '{"kind":"person", "data":"{\"name\":\"bob\"}"}'\
        localhost:3001 catalog.Resources/Create

 }
