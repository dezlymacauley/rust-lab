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

Note:
- I called this `.mise-tasks/runbin.bash` because mise will not allow you
to create a mise task called `run`
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

### Create a Rust project inside the `programs` directory.

Think of this project as a collection of standalone Rust programs.

I'm going to call this `d01-topic-one`, 
and I'm going to create two programs inside of it, called `f01_alpha.rs`,
and `f02_bravo.rs`

```bash
cd programs && cargo new --vcs none d01-topic-one
cd ..

rm -rf programs/d01-topic-one/src
mkdir -p programs/d01-topic-one/src/bin

touch programs/d01-topic-one/src/bin/f01_alpha.rs
touch programs/d01-topic-one/src/bin/f02_bravo.rs
```
_______________________________________________________________________________

### Create a second Rust project inside the `programs` directory.

I'm going to create another Rust project called `d02-topic-two`, 
and I'm going to create two programs inside of it, called `f01_charlie.rs`,
and `f02_delta.rs`

```bash
cd programs && cargo new --vcs none d02-topic-two
cd ..

rm -rf programs/d02-topic-two/src
mkdir -p programs/d02-topic-two/src/bin

touch programs/d02-topic-two/src/bin/f01_charlie.rs
touch programs/d02-topic-two/src/bin/f02_delta.rs
```
_______________________________________________________________________________

### Note: 
- Rust requires every file in the workspace to have a unique name.
- So to be clear you can't have this:
`programs/d02-topic-two/src/bin/f02_delta.rs`
`programs/d01-topic-one/src/bin/f02_delta.rs`
_______________________________________________________________________________

The structure of the `programs` directory should look like this
```
├── programs
│   ├── d01-topic-one
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── bin
│   │           ├── f01_alpha.rs
│   │           └── f02_bravo.rs
│   └── d02-topic-two
│       ├── Cargo.toml
│       └── src
│           └── bin
│               ├── f01_charlie.rs
│               └── f02_delta.rs
```

_______________________________________________________________________________

Add this to the `d01-topic-one/src/bin/f01_alpha.rs` file
```rust
fn main() {
    println!("\nThis is f01_alpha.rs\n");
}
```
_______________________________________________________________________________

Add this to the `d01-topic-one/src/bin/f02_bravo.rs` file
```rust
fn main() {
    println!("\nThis is f02_bravo.rs\n");
}
```
_______________________________________________________________________________

Add this to the `d02-topic-two/src/bin/f01_charlie.rs` file
```rust
fn main() {
    println!("\nThis is f01_charlie.rs\n");
}
```
_______________________________________________________________________________

Add this to the `d02-topic-two/src/bin/f02_delta.rs` file
```rust
fn main() {
    println!("\nThis is f02_delta.rs\n");
}
```
_______________________________________________________________________________

### How to build and run the binary executable of a specify .rs file

E.g. I want to run `f01_alpha.rs`

- First navigate to the directory where the file is.
- Then do this:
`mise runbin f01_alpha.rs`

- Or you can use the `run` alias:
`run f01_alpha.rs`


```bash
cd programs/d01-topic-one/src/bin
```
_______________________________________________________________________________

Run the built-in `cargo clean` command from any directory to save disk space
```bash
cargo clean
```
_______________________________________________________________________________
