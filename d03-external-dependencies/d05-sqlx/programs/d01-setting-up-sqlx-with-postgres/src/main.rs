/*
    Use this command to get cargo-binstall:
    mise use -g cargo-binstall@latest

    To confirm that it is available on your path, run this command:
    cargo-binstall -V

    Use cargo-binstall to install the sqlx-cli:
    cargo binstall --no-confirm sqlx-cli

    Add this to your fish config
    fish_add_path "$HOME/.cargo/bin"

    Add a compose.yaml file at the root of your project:
    ```

    ```
    name: postgres-workflow

    services:
      db:
        container_name: postgres-workflow
        image: postgres:18
        environment:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: password
        ports:
          - "5432:5432"
        volumes:
          - pgdata:/var/lib/postgresql

    volumes:
      pgdata:

*/

fn main() {}
