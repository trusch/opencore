#!/bin/bash

BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/login_admin.sh
source ${BASEDIR}/../common/functions.sh

echo "Create alice@localhost"
payload=$(jq -n '{name: "alice", email: "alice@localhost", password: "password"}')
alice=$(call_opencore idp.Users/Create "$payload")

echo "Create bob@localhost"
payload=$(jq -n '{name: "bob", email: "bob@localhost", password: "password"}')
bob=$(call_opencore idp.Users/Create "$payload")

echo "Create schema 'todo'"
schema=$((cat - | jq -c) <<EOF
{
    "title": "todo",
    "type": "object",
    "properties": {
        "subject": {
            "type":"string"
        },
        "description": {
            "type": "string"
        },
        "status": {
            "type": "string",
            "enum": ["draft", "active", "done"]
        }
    },
    "required": [
        "subject",
        "status"
    ],
    "additionalProperties": false
}
EOF
)
payload=$(jq -n --arg schema "$schema" '{kind: "todo", data: $schema}')
schema=$(call_opencore catalog.Schemas/Create "$payload")

echo "Login as alice"
payload=$(jq -n '{email: "alice@localhost", password: "password"}')
export TOKEN=$(call_opencore idp.Authentication/Login "$payload" | jq -r .accessToken)

echo "Create family group"
payload='{"name":"family"}'
group=$(call_opencore idp.Groups/Create "$payload")
group_id=$(echo $group | jq -r .id)

echo "Add bob to the group"
payload=$(jq -n \
    --arg user_id "$(echo $bob | jq -r .id)" \
    --arg group_id "$(echo $group | jq -r .id)" \
    '{user_id: $user_id, group_id: $group_id}')
call_opencore idp.Groups/AddUser "$payload" >/dev/null

echo "Create a todo item"
resource=$(echo '{"subject": "clean the house", "description": "The house needs cleaning", "status": "active"}' | jq -c)
payload=$(jq -n \
    --arg resource "$resource" \
    '{kind: "todo", data: $resource, labels: {"scope":"house"}, shares: [{principal_id: "family", actions: ["read","write"]}]}')
todo_1=$(call_opencore catalog.Resources/Create "$payload")

echo "Create another todo item"
resource=$(echo '{"subject": "clean the garage", "description": "The garage also needs cleaning", "status": "active"}' | jq -c)
payload=$(jq -n \
    --arg resource "$resource" \
    '{kind: "todo", data: $resource, labels: {"scope":"house"}, shares: [{principal_id: "family", actions: ["read","write"]}]}')
todo_2=$(call_opencore catalog.Resources/Create "$payload")

echo "And one more, this time a personal one"
resource=$(echo '{"subject": "buy beer", "description": "I need to rehydrate after all that cleaning", "status": "active"}' | jq -c)
payload=$(jq -n \
    --arg resource "$resource" \
    '{kind: "todo", data: $resource, labels: {"scope":"personal"}, shares: [{principal_id: "family", actions: ["read","write"]}]}')
todo_3=$(call_opencore catalog.Resources/Create "$payload")

echo "Lets try to create one without a status"
resource=$(echo '{"subject": "buy beer", "description": "I need to rehydrate after all that cleaning"}' | jq -c)
payload=$(jq -n --arg resource "$resource" '{kind: "todo", data: $resource, labels: {"scope":"personal"}}')
call_opencore catalog.Resources/Create "$payload" | jq .

echo "List all todos"
call_opencore catalog.Resources/List '{"kind": "todo"}' | jq -r .data | jq . 

echo "List resources with the scope=house label"
call_opencore catalog.Resources/List '{"kind": "todo", "labels":{"scope": "house"}}' | jq -r .data | jq .

echo "Cleaning the house is done, lets change the status"
id=$(echo $todo_1 | jq -r .id)
patch='{"status":"done"}'
payload=$(jq -n --arg data "$patch" --arg id "$id" '{id: $id, data: $data}')
call_opencore catalog.Resources/Update "$payload" | jq -r .data | jq .

echo "Bob comes home and checks for unfinished tasks"
payload=$(jq -n '{email: "bob@localhost", password: "password"}')
export TOKEN=$(call_opencore idp.Authentication/Login "$payload" | jq -r .accessToken)

echo "List all unfinished tasks related to the house"
payload='{"filter": "$.status != \"done\"", "labels": {"scope": "house"}}'
call_opencore catalog.Resources/List "$payload" | jq -r .data | jq .

# echo "Thats too much, delete all the tasks ( clean up after this demo :) )"
# payload=$(jq -n --arg id "$(echo $todo_1 | jq -r .id)" '{id: $id}')
# call_opencore catalog.Resources/Delete "$payload" >/dev/null
# payload=$(jq -n --arg id "$(echo $todo_2 | jq -r .id)" '{id: $id}')
# call_opencore catalog.Resources/Delete "$payload" >/dev/null
# payload=$(jq -n --arg id "$(echo $todo_3 | jq -r .id)" '{id: $id}')
# call_opencore catalog.Resources/Delete "$payload" >/dev/null