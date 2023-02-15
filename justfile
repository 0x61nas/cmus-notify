#!/usr/bin/env just --justfile

alias r := run
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

# Run the program with all features enabled and the debug profile
run:
  RUST_BACKTRACE=1 RUST_LOG=debug cargo run --all-features

# Run the tests, and genrate a coverage report
coverage:
  CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="target/coverage/data/cargo-test-%p-%m.profraw" cargo test --all-features

# Generate the coverage report
coverage-report: coverage
    grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore "../*" -o ./target/coverage/report/ --llvm --ignore "/*"

    # Open the report in the browser
    xdg-open ./target/coverage/report/index.html
