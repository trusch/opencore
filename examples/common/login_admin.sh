# get admin credentials from logs
eval $(podman logs core 2>&1 | grep OPENCORE_ADMIN | tee .creds)

payload=$(jq -n --arg KEY "$OPENCORE_ADMIN_KEY" '{service_account_id: "root", password: $KEY}')
export TOKEN=$(grpcurl --plaintext -d "$payload" localhost:3001 idp.Authentication/Login | jq -r .accessToken)
