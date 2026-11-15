# Setting up SQLx with Postgres
_______________________________________________________________________________

## Global Setup

Install the `Rust Toolchain` and `Mise`
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

## Project Setup

Add `dotenvy` to the project 
```bash
cargo add dotenvy
```

This is for loading variables declared in the `.env` file into 
the process environment, 
so that their values can be stored as Rust variables.
_______________________________________________________________________________

Add `tokio` to the project, with the `features` flag set to `full`
```bash
cargo add tokio \
    --features full
```
_______________________________________________________________________________

Add `sqlx` to the project
```bash
cargo add sqlx \
    --features runtime-tokio,macros,time,postgres
```
_______________________________________________________________________________

## Objective

1. Create a Postgres cluster called `postgres-workflow`
2. Create a Postgres database inside the cluster that is 
called `ticking_system`
3. Create a table inside the `ticking_system` database,
that is called `tickets`
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

If you want to be explicit when creating a table:
```bash
sqlx database create --database-url \
    postgres://postgres:password@localhost:5432/ticketing_system
```

- Note: I'd avoid this as it exposes the password in the terminal
_______________________________________________________________________________

If you want to be explicit when deleting a table:
```bash
sqlx database drop --database-url \
    postgres://postgres:password@localhost:5432/ticketing_system
```
_______________________________________________________________________________

Use the `sqlx` cli to create a `.sql` file that will store the SQL syntax
required to create the `tickets` table in the `ticking_system` database.
```bash
sqlx migrate add create_tickets
```

You should see an output like this:
```
Creating migrations/20261006160556_create_tickets.sql
```

Note: 
- This file will be blank. That's fine.
- The whole point of this command is to create a `.sql` file that has a
unique index.
_______________________________________________________________________________

Add this to the `migrations/20261006160556_create_tickets.sql` file
```sql
CREATE TABLE tickets (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```
_______________________________________________________________________________

Run this command
```bash
sqlx migrate run
```

You should see an output like this
```
Applied 20261006160556/migrate create tickets (9.960373ms)
```
_______________________________________________________________________________

To confirm if the table was created, run this command:
```bash
docker exec postgres-workflow psql \
    -U postgres \
    -d ticketing_system \
    -c '\dt'
```

You should see an output like this
```
                List of tables
 Schema |       Name       | Type  |  Owner   
--------+------------------+-------+----------
 public | _sqlx_migrations | table | postgres
 public | tickets          | table | postgres
```
_______________________________________________________________________________
