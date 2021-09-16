#!/bin/bash

CALC_TASK=${1:-'scale=1000; pi = 16*a(1/5) - 4*a(1/239); pi'} # pi formula from John Machin (1680–1752)
BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/login_admin.sh
source ${BASEDIR}/../common/functions.sh

echo "Main: Create schema 'calculation'"
schema=$((cat - | jq -c) <<EOF
{
    "title": "calculation",
    "type": "object",
    "properties": {
        "input": {
            "type": "string"
        },
        "output": {
            "type": "string"
        }
    },
    "required": [
        "input"
    ],
    "additionalProperties": false
}
EOF
)
payload=$(jq -n --arg schema "$schema" '{kind: "calculation", data: $schema}')
call_opencore catalog.Schemas/Create "$payload" 2>&1 >/dev/null

echo "Main: create service account 'calculator'"
payload=$(jq -n '{name: "calculator"}')
export CALCULATOR_KEY=$(call_opencore idp.ServiceAccounts/Create "$payload" | jq -r .secretKey)

echo "Main: Create user_1@localhost"
payload=$(jq -n '{name: "user_1", email: "user_1@localhost", password: "password"}')
call_opencore idp.Users/Create "$payload" >/dev/null

function calculation_worker {
    # login calculator
    echo "Worker $1: Login as 'calculator'"
    payload=$(jq -n --arg KEY "$CALCULATOR_KEY" '{service_account_id: "calculator", password: $KEY}')
    export TOKEN=$(call_opencore idp.Authentication/Login "$payload" | jq -r .accessToken)
    
    # listen on calculation events
    echo "Worker $1: Start listening for calculation create events"
    call_opencore catalog.Events/Subscribe '{"resource_kind": "calculation", "event_type": "CREATE"}' | jq -c --unbuffered | while read -r line; do
        echo "Worker $1: There may be a new calculation task!"

        # try to get a lock on that resource
        calc_id=$(echo $line | jq -r .resourceId)
        payload=$(printf '{"lock_id": "%s"}' $calc_id)
        echo "Worker $1: Get a lock with this payload $payload"
        (call_opencore catalog.Locks/TryLock "$payload" | jq -r --unbuffered .lockId | (read -r line && (\
            
            # got lock, get resource to be sure it has no output
            echo "Worker $1: Aquired lock for this resource, refetch to check if the calculation is not ready.";
            payload=$(printf '{"id": "%s"}' $line);
            calc=$(call_opencore catalog.Resources/Get "$payload");
            
            if [[ $(echo $calc | jq -r .data | jq -r .output) == "null" ]]; then
                echo "Worker $1: The calculation task is in fact not ready, lets do it"
                echo "Resource data: $calc"
                data=$(echo $calc | jq -r .data | jq -c .)
                echo "Task data: $data"
                input=$(echo $data | jq -r .input)
                
                # the actual work, offloaded to `bc`
                echo "Worker $1: Calculate..."
                echo "$input | BC_LINE_LENGTH=0 bc -l"
                output=$(echo $input | BC_LINE_LENGTH=0 bc -l);
                
                # save the result by patching the resource
                echo "Worker $1: Send result"
                patch=$(jq -c -n --arg output $output '{output: $output}')
                payload=$(jq -cn --arg id "$calc_id" --arg data "$patch" '{id: $id, data: $data}')
                echo "payload: $payload"
                call_opencore catalog.Resources/Update "$payload" >/dev/null;
            else
                echo "Worker $1: The calculation task is already done in the meantime, nothing to do here."
            fi

            echo "Worker $1: Release lock"
        ))) || echo "Worker $1: Failed to get lock"
    done
    echo "Worker $1: Exit"
}

echo "Main: Start workers"
calculation_worker worker_1 &
calculation_worker worker_2 &
calculation_worker worker_3 &

echo "Main: Login as user_1"
payload=$(jq -n '{email: "user_1@localhost", password: "password"}')
export TOKEN=$(call_opencore idp.Authentication/Login "$payload" | jq -r .accessToken)

echo "Main: Create calculation resource with input '${CALC_TASK}'"
data=$(printf '{"input": "%s"}' "${CALC_TASK}" | jq -c)
payload=$(jq -n --arg data "$data" '{kind: "calculation", data: $data, shares: [{principal_id: "calculator", actions: ["read", "write"]}]}')

start_ms=$(($(date +%s%N)/1000000)) # timestamp in milliseconds

calculation=$(call_opencore catalog.Resources/Create "$payload")
calculation_id=$(echo $calculation | jq -r .id)
echo "Main: new calculation id: $calculation_id"
echo $calculation

payload=$(printf '{"resource_id": "%s", "event_type": "UPDATE"}' $calculation_id)
echo "Main: listen for update events on $calculation_id"
call_opencore catalog.Events/Subscribe "$payload" | jq -c --unbuffered | (read -r line && (\
    echo "Main: Got the result:"
    echo $line | jq -r .data | jq .
    end_ms=$(($(date +%s%N)/1000000)) # timestamp in milliseconds
    echo "Main: waiting for this took $(echo "$end_ms - $start_ms" | bc -l)ms"
))