#!/usr/bin/env bash
#MISE description="🔎 Query the database with pgcli | alias = connect-query"
#MISE quiet=true
#______________________________________________________________________________

# STEP: 1 => Ensure that a valid Postgres cluster exists and is running

set -euo pipefail

: "${PGDATA:?Missing environment variable PGDATA}"
: "${PGUSER:?Missing environment variable PGUSER}"
: "${PGPASSWORD:?Missing environment variable PGPASSWORD}"
: "${PGHOST:?Missing environment variable PGHOST}"
: "${PGPORT:?Missing environment variable PGPORT}"
: "${PGDATABASE:?Missing environment variable PGDATABASE}"
: "${PGCLI_CONFIG_FILE:?Missing environment variable PGCLI_CONFIG_FILE}"

if [[ ! -f "$PGDATA/PG_VERSION" ]]; then
    printf "❌ No valid Postgres cluster was found.\n" >&2
    exit 1
fi

if ! pg_ctl -D "$PGDATA" status > /dev/null 2>&1; then
    printf "❌ The Postgres server is not running.\n" >&2
    exit 1
fi
#______________________________________________________________________________

# STEP: 2 => Check if the database and user exist

printf "\n"
printf "⏳ Checking database and user...\n"

existence_check="$(
    psql \
        --host="$PGHOST" \
        --port="$PGPORT" \
        --username="$PGUSER" \
        --dbname=postgres \
        --no-psqlrc \
        --no-password \
        --quiet \
        --tuples-only \
        --no-align \
        --set=ON_ERROR_STOP=1 \
        --set="database_name=$PGDATABASE" \
        --set="user_name=$PGUSER" \
        --file=- <<'SQL'
            SELECT
                EXISTS (
                    SELECT FROM pg_database
                    WHERE datname = :'database_name'
                ),
                EXISTS (
                    SELECT FROM pg_roles
                    WHERE rolname = :'user_name'
                );
SQL
)"

IFS='|' read -r database_exists user_exists <<< "$existence_check"

if [[ "$database_exists" != "t" ]]; then
    printf "❌ Database '%s' does not exist.\n" "$PGDATABASE" >&2
    exit 1
fi

if [[ "$user_exists" != "t" ]]; then
    printf "❌ User '%s' does not exist.\n" "$PGUSER" >&2
    exit 1
fi
#______________________________________________________________________________

# STEP: 3 => Connect to Postgres using pgcli

printf "\n"
printf "⏳ Connecting to Postgres...\n"

exec pgcli \
    --pgclirc "$PGCLI_CONFIG_FILE" \
    --host "$PGHOST" \
    --port "$PGPORT" \
    --username "$PGUSER" \
    --dbname "$PGDATABASE" \
    --no-password
#______________________________________________________________________________
