#!/bin/bash

BASEDIR=$(dirname "$0")

source ${BASEDIR}/../common/functions.sh
source ${BASEDIR}/../common/login_admin.sh

echo "Start workers in parallel"
function do_work {
    call_opencore catalog.Locks/Lock '{"lock_id": "test"}' | jq -c --unbuffered | {
        read line;
        echo $line;
        for i in $(seq 1 100); do 
            echo $1 says $i
        done
    }
}

for i in $(seq 1 10); do
    do_work worker_$i &
done

wait