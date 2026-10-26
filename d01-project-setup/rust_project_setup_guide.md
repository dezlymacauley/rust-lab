# Rust Project Setup Guide
_______________________________________________________________________________

Create the project directory
```bash
mkdir rust-project
```
_______________________________________________________________________________

Enter the project directory
```bash
cd rust-project
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

Create the rest of the project structure
```bash
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

Replace the contents of the `src/main.rs` file with this
```rust
fn main() {
    println!("\nRust Project\n");
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

To run the project, all you need is one command:
```bash
run
```
_______________________________________________________________________________
