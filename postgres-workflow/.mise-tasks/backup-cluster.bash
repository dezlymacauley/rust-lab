#!/usr/bin/env bash
#MISE description="📸 Backup the Postgres Cluster | alias = backup-cluster"
#MISE quiet=true
#______________________________________________________________________________

# STEP: 1 => Ensure that the relevant environment variables are set

set -euo pipefail

: "${PGDATA:?Missing environment variable PGDATA}"
: "${PGHOST:?Missing environment variable PGHOST}"
: "${PGPORT:?Missing environment variable PGPORT}"
: "${PGPASSWORD:?Missing environment variable PGPASSWORD}"
: "${PG_BACKUPS_DIR:?Missing environment variable PG_BACKUPS_DIR}"
#______________________________________________________________________________

# STEP: 2 => Check if a valid Postgres cluster exists and the server is running

if [[ ! -f "$PGDATA/PG_VERSION" ]]; then
    printf "❌ No valid Postgres cluster was found.\n" >&2
    exit 1
fi

if ! pg_ctl -D "$PGDATA" status > /dev/null 2>&1; then
    printf "❌ The Postgres server is not running.\n" >&2
    exit 1
fi
#______________________________________________________________________________

# STEP: 3 => Prepare the backup directory

# Generate a UTC timestamp using ISO 8601 format.
timestamp="$(date -u +%Y_%m_%dT%H_%M_%SZ)"

# Create the backup directory if it doesn't exist.
mkdir -p -- "$PG_BACKUPS_DIR"

# Restrict backup file permissions.
umask 077
#______________________________________________________________________________

# STEP: 4 => Create a logical backup of the Postgres cluster

printf "\n"
printf "⏳ Creating logical Postgres cluster backup...\n"

backup_file="$PG_BACKUPS_DIR/postgres_cluster_$timestamp.sql"

if ! pg_dumpall \
    --host="$PGHOST" \
    --port="$PGPORT" \
    --username=postgres \
    --no-password \
    --file="$backup_file"; then

    rm -f -- "$backup_file"

    printf "❌ Failed to create logical Postgres cluster backup.\n\n" >&2
    exit 1
fi

printf "\n✅ Logical Postgres cluster backup created:\n"
printf "%s\n\n" "$backup_file"
#______________________________________________________________________________
