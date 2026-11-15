# The Rust Module System
_______________________________________________________________________________

## A High-Level Overview

- A `package` is simply the technical the term for a Rust project,
regardless of what type of Rust project it is.

- So `rust project` = `rust package`

```
Package
└── Crate(s)
    └── Module(s)
        └── Item(s)
```
_______________________________________________________________________________

- Every `package` contains at least one `crate` 
- Every `crate` contains at least one `module`
- Every `module` contains at least one `item`

_______________________________________________________________________________

## What is `Cargo.toml`?

- Every `package` contains a `Cargo.toml` file at the root of the directory,
to indicate that the directory is a Rust project.

- There are two ways to create a new package (aka a new Rust project).

- For this guide, I'm going to create a project called `cyber-punk`
_______________________________________________________________________________

### Method 1 - Create a new directory that contains a `Cargo.toml` file

This is done using the `cargo new` command

```bash
cargo new cyber-punk
cd cyber-punk
```
_______________________________________________________________________________

### Method 2 - Add a Cargo.toml file to the current directory

This is done using the `cargo init` command

```bash
mkdir cyber-punk
cd cyber-punk
cargo init
```
_______________________________________________________________________________

Regardless of which method you used, you should have a `Cargo.toml` file
that looks like this.

```toml
[package]
name = "cyber-punk"
version = "0.1.0"
edition = "2024"

[dependencies]
```
_______________________________________________________________________________

## What is a `crate`?

- Every Rust package must contain at least one crate.
- There are three types of crates
_______________________________________________________________________________

### Crate Type 1: Single binary crate

When you create a new Rust project using `cargo  new` or `cargo init`,
and you don't specify the project type, Cargo will automatically create
a `binary crate` in the directory.

```
.
├── Cargo.toml
└── src
    └── main.rs
```

The root of the crate is `src/main.rs`

```rust
fn main() {
    println!("\nCyber Punk - Program 01\n");
}
```
_______________________________________________________________________________

### Crate Type 2: Multiple binary crate

To create a package with a multiple binary create,
use `cargo new project-name` or `cargo init project-name`

Then delete the `src/main.rs` file
```bash
rm src/main.rs
```

Create a `src/bin/` directory and place each program in that directory
```bash
mkdir -p src/bin
touch src/bin/program_01.rs
touch src/bin/program_02.rs
```

Each file in `src/bin/` is a binary crate. 

src/bin/program_01.rs
```rust
fn main() {
    println!("\nCyber Punk - Program 01\n");
}
```

src/bin/program_02.rs
```rust
fn main() {
    println!("\nCyber Punk - Program 02\n");
}
```
_______________________________________________________________________________

### Crate Type 3: Library crate

To create a library crate use the `--lib` flag.

So that's `cargo new --lib library-name` or `cargo init --lib library-name`

The root of the crate is `src/lib.rs`
```rust
pub fn print_library_name() {
    println!("Cyber Punk - Library");
}
```

The biggest difference is that a library crate does not have 
a `main` function. This is because a library create is meant to be imported
into another program (and that program would have a main function).
_______________________________________________________________________________

## What is a `module`?

#### A quick recap
- Every `crate` in Rust must have at least one module.
- For a single binary crate, that module is file `src/main.rs`
- For a multiple binary crate, each file inside the `src/bin` directory
is a module.
- For a library crate, that module is `src/lib.rs`

#### The simplest explanation of a module
- A module is simply file that can contain Rust items like a functions,
structs, enums and many more.
- Modules allow you to structure your code into smaller parts,
so that you don't have one gigantic file with all your code.
_______________________________________________________________________________
