# Setting up SQLx with Postgres
_______________________________________________________________________________

## Glocal Requirements
1. rust
2. mise
3. cargo-binstall 
4. sqlx-cli
5. docker
_______________________________________________________________________________

Use mise to install `cargo-binstall` globally
```bash
mise use -g cargo-binstall
```
_______________________________________________________________________________

Use `cargo-binstall` to to install the `sqlx-cli` globally
```bash
cargo binstall --no-confirm sqlx-cli
```
_______________________________________________________________________________

## Objective

1. Create a Postgres cluster called `postgres-workflow`
2. Create a Postgres database inside the cluster that is 
called `ticking_system`
3. Create a table inside the `ticking_system` database,
that is called `tickets`

_______________________________________________________________________________

Create a `compose.yaml` file to the root of your project
```bash
touch compose.yaml
```
_______________________________________________________________________________

Add this to the file
```bash
name: postgres-workflow

services:
  db:
    container_name: postgres-workflow
    image: postgres:18.6
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql

volumes:
  pgdata:
```
_______________________________________________________________________________

Ensure that Docker is running, 
then run this command at the root of your project:
```bash
docker compose up -d
```
_______________________________________________________________________________

Run this command to confirm that the Postgres Cluster 
called `postgres-workflow` is ready to accept connections:
```bash
docker exec postgres-workflow pg_isready
```

You should see this:
```
/var/run/postgresql:5432 - accepting connections
```
_______________________________________________________________________________

Add these lines to the end of the `.gitignore` file at the root 
of your project:

```gitignore
# Environment Variables
.env
```
_______________________________________________________________________________

Add this to the `.env` file
```bash
DATABASE_URL=postgres://postgres:password@localhost:5432/ticketing_system
```

Note: The `ticketing_system` database does not exist yet, and that is fine.
_______________________________________________________________________________

Use the `sqlx` cli to create the database
```bash
sqlx database create
```

This command will use the `DATABASE_URL` environment variable to create
the database. If the database already exists, nothing will happen.
_______________________________________________________________________________

To see if the database was created
```bash
docker exec postgres-workflow psql -U postgres -x -c '\l'
```
_______________________________________________________________________________

If you ever want to delete the database
```bash
sqlx database drop
```
_______________________________________________________________________________

Note the commands `sqlx database create` and `sqlx database drop` will
automatically read the `DATABASE_URL` variable from `.env` file. 

You don't need to load the `.env` when using these commands
_______________________________________________________________________________

However I prefer to be explicity

_______________________________________________________________________________
