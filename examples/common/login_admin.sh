# get admin credentials from logs
CONTAINER_ENGINE=${CONTAINER_ENGINE:-podman}

if [ "$CONTAINER_ENGINE" = "podman" ]; then
    eval $($CONTAINER_ENGINE logs core 2>&1 | grep OPENCORE_ADMIN | tee .creds)
else 
    eval $($CONTAINER_ENGINE logs opencore_opencore_1 2>&1 | grep OPENCORE_ADMIN | tee .creds)
fi

payload=$(jq -n --arg KEY "$OPENCORE_ADMIN_KEY" '{service_account_id: "root", password: $KEY}')
export TOKEN=$(grpcurl --plaintext -d "$payload" localhost:3001 idp.Authentication/Login | jq -r .accessToken)
