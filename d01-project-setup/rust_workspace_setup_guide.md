# Go Workspace Setup Guide
_______________________________________________________________________________

Create the workspace directory
```bash
mkdir go-workspace
```
_______________________________________________________________________________

Enter the workspace directory
```bash
cd go-workspace
```
_______________________________________________________________________________

Set the project to use the latest version of `go` (Go compiler)
```bash
mise use go@latest
```

#### Note:
- If you want to use a specific version you can use mise to check

- To get the version number of the latest available Go 
that you can use in a project, run this command: `mise latest go`

- If you want to view a list of valid Go versions that you can use,
run this command: `mise ls-remote go`
_______________________________________________________________________________

Set the project to use the latest version of `eza` (project structure viewer)
```bash
mise use eza@latest
```
_______________________________________________________________________________

Update the `mise.toml` file in the project to look like this
```toml
[tools]
eza = "latest"
go = "latest"
#______________________________________________________________________________

[tasks.build]
description = "👷 Build the current program | alias = build"
quiet = true
run = """

set -e
name=$(basename "$MISE_ORIGINAL_CWD")

if [ ! -d "programs/${name}/cmd/${name}" ]; then
    printf "\n%s\n\n" "❌ Not inside a program. cd into a program first"
    exit 1
fi

mkdir -p bin
go build -o "bin/${name}" "./programs/${name}/cmd/${name}"
printf "\n%s\n\n" "✅ Built: bin/${name}"

"""
#______________________________________________________________________________

[tasks.clean]
description = "🧼 Delete build output and cache | alias = clean"
quiet = true
run = """

rm -rf bin
go clean -cache

"""
#______________________________________________________________________________

[tasks.format]
description = "📜 Run the code formatter on the workspace | alias = format"
quiet = true
run = """

gofmt -w .

"""
#______________________________________________________________________________

[tasks.run-bin]
description = "🤖 Build and run the current program's binary | alias = run-bin"
depends = ["build"]
quiet = true
run = """

"./bin/$(basename "$MISE_ORIGINAL_CWD")"

"""
#______________________________________________________________________________

[tasks.setup]
description = "🚀 Tidy the dependencies of every program | alias = setup"
quiet = true
run = """

set -e

for dir in programs/*/; do
    [ -f "${dir}go.mod" ] || continue
    (cd "$dir" && go mod tidy)
done

printf "\n%s\n\n" "✅ Dependencies are ready"

"""
#______________________________________________________________________________

[tasks.structure]
description = "🌲 View the project structure | alias = structure"
quiet = true
run = """

eza --tree --git-ignore

"""
#______________________________________________________________________________

[shell_alias]
build = "mise run build"
clean = "mise run clean"
format = "mise run format"
run = "go run ./cmd/*"
run-bin = "mise run run-bin"
setup = "mise run setup"
structure = "mise run structure"
#______________________________________________________________________________
```
_______________________________________________________________________________

Create a `.gitignore` file
```sh
touch .gitignore
```
_______________________________________________________________________________

Add this to the `.gitignore` file

```gitignore
# Build Output
bin/
```
_______________________________________________________________________________

Create a `programs` directory
```bash
mkdir programs
```
_______________________________________________________________________________

Intialize the workspace
```bash
go work init
```

#### Note:
- This will create a `go.work` file that contains that version of Go that
will be used in the workspace
_______________________________________________________________________________

To confirm that your workspace is using the correct version of Go, 
run this:
```sh
go version
which go
```
_______________________________________________________________________________

You should see an output similar to this:
```
go version go1.27.3 linux/amd64
/home/dezlymacauley/.local/share/mise/installs/go/1.27.3/bin/go
```
_______________________________________________________________________________

## How to add programs to the workspace
_______________________________________________________________________________

In this guide I'm going to add two programs, which I will call `d01` and `d02`

All of the programs in the workspace will share the same Go version.
_______________________________________________________________________________

This is how to add the first program, `d01`
```bash
mkdir -p programs/d01/cmd/d01
touch programs/d01/cmd/d01/main.go

cd programs/d01
    go mod init d01
cd ../..

go work use ./programs/d01
```
_______________________________________________________________________________

This is how to add the second program, `d02`
```bash
mkdir -p programs/d02/cmd/d02
touch programs/d02/cmd/d02/main.go

cd programs/d02
    go mod init d02
cd ../..

go work use ./programs/d02
```
_______________________________________________________________________________

Add this to the `programs/d01/cmd/d01/main.go` file
```go
package main

import "fmt"

func main() {
	fmt.Println()
	fmt.Println("This is d01")
	fmt.Println()
}
```
_______________________________________________________________________________

Add this to the `programs/d02/cmd/d02/main.go` file
```go
package main

import "fmt"

func main() {
	fmt.Println()
	fmt.Println("This is d02")
	fmt.Println()
}
```
_______________________________________________________________________________

Run this command to see the list of `mise tasks` in your project
```bash
mise tasks
```
_______________________________________________________________________________

You should see an output like this
```
Name       Description
build      👷 Build the current program | alias = build
clean      🧼 Delete build output and cache | alias = clean
format     📜 Run the code formatter on the workspace | alias = format
run-bin    🤖 Build and run the current program's binary | alias = run-bin
setup      🚀 Tidy the dependencies of every program | alias = setup
structure  🌲 View the project structure | alias = structure
```
_______________________________________________________________________________

To view the project structure
```bash
structure
```

This should be the structure
```
.
├── go.work
├── mise.toml
└── programs
    ├── d01
    │   ├── cmd
    │   │   └── d01
    │   │       └── main.go
    │   └── go.mod
    └── d02
        ├── cmd
        │   └── d02
        │       └── main.go
        └── go.mod
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

To build a program, 
navigate to its directory and use the `build` command

E.g. To build the program `d01`
```bash
cd programs/d01
build
```
#### Note:
- I've set this up so that all of built programs, 
are stored in the `bin` directory at the root of the workspace.
_______________________________________________________________________________

To build a program and run the built binary, 
navigate to its directory and use the `run-bin` command
```bash
cd programs/d01
run-bin
```
_______________________________________________________________________________
