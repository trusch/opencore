BASEDIR=$(dirname "$0")

source ${BASEDIR}/login_admin.sh
source ${BASEDIR}/functions.sh

call_opencore catalog.Events/Subscribe "{}" | jq -c --unbuffered | jq .