#!/usr/bin/env bash
#MISE description="🐘 Toggle the Postgres Server | alias = toggle-server"
#MISE quiet=true
#______________________________________________________________________________

# STEP: 1 => Ensure that the relevant environment variables are set

set -euo pipefail

: "${PGDATA:?Missing environment variable PGDATA}"
: "${PGHOST:?Missing environment variable PGHOST}"
: "${PGPORT:?Missing environment variable PGPORT}"

if [[ ! -f "$PGDATA/PG_VERSION" ]]; then
    printf "❌ No valid Postgres cluster was found.\n" >&2
    exit 1
fi
#______________________________________________________________________________

# STEP: 2 => Check the Postgres server status

if pg_ctl -D "$PGDATA" status > /dev/null 2>&1; then

    printf "\n"
    printf "⏳ Stopping the Postgres server...\n\n"

    # `-m fast` terminates existing connections
    # and rolls back active transactions.
    #
    # `-w` waits until the server has completely stopped.
    pg_ctl -D "$PGDATA" stop -m fast -w > /dev/null

    printf "✅ Postgres server stopped.\n\n"

else

    printf "\n"
    printf "⏳ Starting the Postgres server...\n\n"

    pg_ctl -D "$PGDATA" -l "$PGDATA/server.log" start -w > /dev/null

    printf "✅ Postgres server started successfully:\n"
    printf "%s:%s\n\n" "$PGHOST" "$PGPORT"
fi
#______________________________________________________________________________
