function call {
    grpcurl --plaintext -H "Authorization: Bearer ${TOKEN}" -d "$3" "$1" "$2"
}

function call_opencore {
    call localhost:3001 "$1" "$2"
}