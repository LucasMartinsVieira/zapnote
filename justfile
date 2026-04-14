# List all just recipes 
@default:
  @just --list

# Run all unit tests using cargo.
test:
  cargo test

# Format all Rust code.
fmt:
  cargo fmt --all

# Run all the "lint" recipes
lint-all:
  @just lint
  @just msrv

# Lint and verify code with formatting and Clippy checks. 
lint:
  cargo check
  cargo fmt --all --check
  cargo clippy --all-features --all-targets -- -Dwarnings

# Checks Minimum Supported Rust Version (MSRV)
msrv:
  cargo msrv verify
