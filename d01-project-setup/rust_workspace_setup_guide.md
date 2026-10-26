# Rust Workspace Setup Guide
_______________________________________________________________________________

Create the project directory
```bash
mkdir rust-workspace
```
_______________________________________________________________________________

Enter the project directory
```bash
cd rust-workspace
```
_______________________________________________________________________________

Initialize the project
```bash
cargo init --vcs none
```
#### Note:
- `--vcs none` tell cargo that I don't want a version control system to be
setup for me. I prefer to setup git myself when I'm ready.

_______________________________________________________________________________

Set the project to use the latest version of `eza` (project structure viewer)
```bash
mise use eza@latest
```
_______________________________________________________________________________

Create the rest of the workspace structure
```bash
rm -rf src

mkdir programs
touch .gitignore
touch rust-toolchain.toml
touch rustfmt.toml
```
_______________________________________________________________________________

Add this to the `.gitignore` file
```gitignore
# Build Output
/target/
```
_______________________________________________________________________________

Update the `Cargo.toml` file to look like this
```toml
[workspace]
resolver = "3"
members = [
    "programs/*"
]
```
_______________________________________________________________________________

Update the `mise.toml` file in the project to look like this
```toml
[tools]
eza = "latest"
#______________________________________________________________________________

[shell_alias]
build = "cargo build"
clean = "cargo clean"
format = "cargo fmt"
run = "cargo run --quiet"
structure = "eza --tree --git-ignore"
#______________________________________________________________________________
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

## How to add programs to the workspace

_______________________________________________________________________________

In this guide I'm going to add two programs, which I will call `d01` and `d02`

All of the programs in the workspace will share the same rustc version.
_______________________________________________________________________________

This is how to add the first program, `d01`
```bash
cargo new programs/d01
```
_______________________________________________________________________________

This is how to add the second program, `d02`
```bash
cargo new programs/d02
```
_______________________________________________________________________________

Replace the contents of the `programs/d01/src/main.rs` file with this
```rust
fn main() {
    println!("\nThis is d01\n");
}
```
_______________________________________________________________________________

Replace the contents of the `programs/d02/src/main.rs` file with this
```rust
fn main() {
    println!("\nThis is d02\n");
}
```
_______________________________________________________________________________

To view the project structure
```bash
structure
```

This should be the structure
```
.
├── Cargo.lock
├── Cargo.toml
├── mise.toml
├── programs
│   ├── d01
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── d02
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── rust-toolchain.toml
├── rustfmt.toml
└── src
    └── main.rs
```
_______________________________________________________________________________

To view the list of shell aliases that were declared in the `[shell_alias]`
section of the `mise.toml` file, just run this command
```bash
mise shell-alias ls
```

You should see this output
```
alias     command
build     cargo build
clean     cargo clean
format    cargo fmt
run       cargo run --quiet
structure eza --tree --git-ignore
```
_______________________________________________________________________________

To run a program, 
navigate to the directory and use the `run` command

E.g. To run the program `d01`
```bash
cd programs/d01
run
```
_______________________________________________________________________________
