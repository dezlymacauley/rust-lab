# 🐘 Postgres Workflow
_______________________________________________________________________________

## Setup

If you ever want to setup your workflow after cloning this directory,
or moving it to a different location, just run the three commands below:
```sh
mise trust
chmod +x .mise-tasks/*.bash
mise install
```
_______________________________________________________________________________

## Usage

Run this command to view a list of available commands:
```bash
menu
```

You should see this
```
Name            Description
backup-cluster  📸 Backup the Postgres Cluster | alias = backup-cluster
check-status    🚥 Check the status of Postgres | alias = check-status
connect-admin   📜 Connect as admin user with pqsl | alias = connect-admin
setup-cluster   🏡 Setup the Postgres Cluster | alias = setup-cluster
toggle-server   🐘 Toggle the Postgres Server | alias = toggle-server

alias          command
backup-cluster mise run backup-cluster
check-status   mise run check-status
connect-admin  mise run connect-admin
menu           mise tasks; echo; mise shell-alias ls
setup-cluster  mise run setup-cluster
toggle-server  mise run toggle-server
```
_______________________________________________________________________________

### NOTE: Using `connect-admin`

To use `connect-admin`:
1. Ensure that the `PGUSER` in your .env file is set to `postgres`,
and that `PGPASSWORD` is set to the password for Postgres.

2. `connect-admin` is designed to use `psql` 
to connect to the `postgres` database, using the user `postgres`,
to perform administrative tasks on the Postgres cluster.
_______________________________________________________________________________

### NOTE: Using `restore-cluster`

To use `restore-cluster`:
1. Create a backup of your current cluster with `backup-cluster`

2. Then run `setup-cluster` to delete the current current cluster and
setup a new postgres cluster.

3. Then run `restore-cluster` to replace this cluster with 
the`.sql` backup file that was created when you 
ran the `backup-cluster` command.
_______________________________________________________________________________
