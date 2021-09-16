#!/bin/bash

BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/login_admin.sh
source ${BASEDIR}/../common/functions.sh

cp .creds .list-creds

echo "Create schema 'animal'"
schema=$((cat - | jq -c) <<EOF
{
    "title": "animal",
    "type": "object",
    "properties": {
        "name": {
            "type":"string"
        },
        "species": {
            "type": "string"
        },
        "age": {
            "type": "integer"
        }
    },
    "required": [
        "name",
        "species"
    ],
    "additionalProperties": false
}
EOF
)

payload=$(jq -n --arg schema "$schema" '{kind: "animal", data: $schema}')
call_opencore catalog.Schemas/Create "$payload" | jq .

user_id=$(echo $TOKEN | cut -d. -f2 | base64 -d | jq -r .sub)
export user_id="'$user_id'"

COUNT=100000
echo "Create $COUNT animals"

rm data.sql

species_list[1]="ant"
species_list[2]="zebra"
species_list[3]="gorilla"
species_list[4]="lion"

size=${#species_list[@]}
index=$(($RANDOM % $size))
echo ${array[$index]}
for i in $(seq 1 $COUNT); do
    index=$((($RANDOM % $size)+1))
    species=$(echo ${species_list[$index]})
    name=$(namegen)
    cat >> data.sql <<EOF
insert into resources (resource_id, creator_id, kind, data, labels) VALUES(uuid_generate_v4(), uuid_generate_v4(), 'animal', ('{"age": $(($RANDOM % 100)) , "name": "${name}", "species": "${species}"}')::jsonb, '{}'::jsonb);
EOF
done

PGPASSWORD=password psql -U postgres -h localhost <<EOF
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
EOF

PGPASSWORD=password psql -U postgres -h localhost < data.sql >/dev/null

PGPASSWORD=password psql -U postgres -h localhost <<EOF
update resources set permission_parent_id = resource_id;
insert into permissions (action, principal_id, resource_id) SELECT 'read', ${user_id}, resource_id FROM resources;
EOF

