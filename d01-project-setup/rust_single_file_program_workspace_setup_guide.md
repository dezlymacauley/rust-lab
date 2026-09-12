# Rust Single-File Program Workspace Setup Guide
_______________________________________________________________________________

### Create the project directory and enter it
_______________________________________________________________________________

```bash
mkdir rust-single-file-program-workspace
cd rust-single-file-program-workspace
```
_______________________________________________________________________________

Initialize the project
```bash
cargo init --vcs none
```

Note:
- `--vcs none` tell cargo that I don't want a version control system to be
setup for me. I prefer to setup git myself when I'm ready.
_______________________________________________________________________________

### Create the rest of the project structure

```bash
touch .gitignore
touch mise.toml && mise trust
touch rust-toolchain.toml
touch rustfmt.toml

mkdir .mise-tasks
cd .mise-tasks && touch runbin.bash
cd ..
chmod u+x .mise-tasks/*.bash

rm -rf src
mkdir programs
```
_______________________________________________________________________________

Add this to the `.mise-tasks/runbin.bash` 
```bash
#!/usr/bin/env bash

#MISE description="🤖 Run the binary of a .rs file | alias = run"
#MISE quiet=true

BINARY_NAME=$(basename "$1" .rs)

if [ -z "$1" ]; then
    printf "\n%s\n" '❌ Error:'
    printf "%s\n\n" 'You did not specify which .rs file to run'
    printf "%s\n" 'Usage:'
    printf "%s\n\n" 'mise runbin f01_example_file.rs'
    printf "%s\n" 'Or use the run alias:'
    printf "%s\n\n" 'run f01_example_file.rs'
    exit 0
fi

cargo run --quiet --bin "$BINARY_NAME"
```
_______________________________________________________________________________

Add this to the `.gitignore` file
```gitignore
# Build Output
/target/
```
_______________________________________________________________________________

Replace the contents of the `Cargo.toml` file with this
```toml
[workspace]
resolver = "3"
members = [
    "programs/*"
]
```
_______________________________________________________________________________

Add this to the `mise.toml` file
```toml
[shell_alias]
run = "mise runbin"
```
_______________________________________________________________________________

Add this to the `rust-toolchain.toml` file
```toml
[toolchain]
channel = "stable"
components = [ "rust-analyzer" ]
```
_______________________________________________________________________________

Add this to the `rustfmt.toml` file
```toml
max_width = 80
tab_spaces = 4
```
_______________________________________________________________________________

Create a Rust project inside the `programs` directory.

Think of this project as a collection of standalone Rust programs.

I'm going to call this `d01-topic-one`, 
and I'm going to create two programs inside of it, called `f01_alpha.rs`,
and `f02_bravo.rs`

```bash
cd programs && cargo new --vcs none d01-topic-one
cd ..

rm -rf programs/d01-topic-one/src
mkdir -p programs/d01-topic-one/src/bin

cd programs/d01-topic-one/src/bin
touch f01_alpha.rs
touch f02_bravo.rs
cd ..
```
_______________________________________________________________________________


Replace the contents of the `src/main.rs` file with this
```rust
fn main() {
    println!("\nRust Project\n");
}
```
_______________________________________________________________________________

### Build the program (Create an executable binary)

```bash
cargo build
```
_______________________________________________________________________________

### Run the program (Run the executable binary)

```bash
cargo dev
```
_______________________________________________________________________________

Note:
- `cargo` has a built-in `clean` command to delete the `target` directory

```bash
cargo clean
```
_______________________________________________________________________________
