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
