# Rust Project Setup Guide
_______________________________________________________________________________

### Create the project directory and enter it
_______________________________________________________________________________

```bash
mkdir rust-project && cd rust-project 
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
touch rust-toolchain.toml
touch rustfmt.toml

mkdir .cargo
touch .cargo/config.toml
```
_______________________________________________________________________________

Add this to the `.gitignore` file
```gitignore
# Build Output
/target/
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

Add this to the `.cargo/config.toml` file
```toml
[alias]
dev = "run --quiet"
```

Note:
- Now `cargo dev` is the same as `cargo run --quiet`
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
