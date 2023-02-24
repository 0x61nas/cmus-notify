#!/usr/bin/env just --justfile

alias b := build
alias br := build-release
alias r := run
alias h := show-help
alias t := test
alias l := lint
alias c := check
alias cov := coverage-report

default: lint test run

# Test the program with all features enabled.
test:
  cargo test --all-features

# Check the program with all features enabled.
check:
  cargo check --all-features

lint:
  cargo clippy --all-features --all-targets -- -D warnings

# Build the program with all features enabled
build:
  cargo build --all-features

# Build the program with all features enabled in release mode
build-release:
  cargo build --all-features --release

# Run the program with all features enabled and the debug profile
run: build
  RUST_BACKTRACE=1 RUST_LOG=debug cargo run --all-features

# Run the program with all features enabled and use the `--help` flag
show-help:
  cargo run --all-features -- --help

# Run the tests, and genrate a coverage report
coverage:
  CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="target/coverage/data/cargo-test-%p-%m.profraw" cargo test --all-features

# Generate the coverage report
coverage-report: coverage
    # Generate the report in html format using grcov
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore "../*" -o ./target/coverage/report/ --llvm --ignore "/*"

    # Open the report in the browser
    xdg-open ./target/coverage/report/index.html

remove-config:
    rm -rf ~/.config/cmus-notify/

markdown-help: build
    cargo run --all-features -- --markdown-help > docs/usage.md
