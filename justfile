#!/usr/bin/env just --justfile

REPO_NAME := 'cmus-notify'

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

setup-remotes:
    git remote add github git@github.com:0x61nas/{{REPO_NAME}}.git
    git remote add gitlab git@gitlab.com:anelgarhy/{{REPO_NAME}}.git
    git remote add codeberg ssh://git@codeberg.org/0x61nas/{{REPO_NAME}}.git
    git remote add disroot ssh://git@git.disroot.org/anas/{{REPO_NAME}}.git
    git remote add tangled git@tangled.org:anas.tngl.sh/{{REPO_NAME}}
    git remote add codefloe ssh://git@codefloe.com/anas/{{REPO_NAME}}.git

# Push the code to all remotes
push FLAGS="-u" BRANSH="aurora":
    git push {{FLAGS}} github {{BRANSH}}
    git push {{FLAGS}} gitlab {{BRANSH}}
    git push {{FLAGS}} codeberg {{BRANSH}}
    git push {{FLAGS}} disroot {{BRANSH}}
    git push {{FLAGS}} tangled {{BRANSH}}
    git push {{FLAGS}} codefloe {{BRANSH}}

# Push the git tags to all remotes
pusht: push
    git push --tags github
    git push --tags gitlab
    git push --tags codeberg
    git push --tags disroot
    git push --tags tangled
    git push --tags codefloe

clean:
    git clean -ffdx
