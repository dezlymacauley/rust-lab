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
