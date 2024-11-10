# List all just recipes 
@default:
  @just --list

# Run all unit tests using cargo.
test:
  cargo test

# Format all Rust and Nix code.
fmt:
  cargo fmt --all
  nixfmt -- *.nix

# Run all the "lint" recipes
lint-all:
  @just lint
  @just msrv

# Lint and verify code with formatting, Clippy checks, and Nix file syntax.
lint:
  cargo check
  cargo fmt --all --check
  cargo clippy --all-features --all-targets -- -Dwarnings
  nixfmt --check -- *.nix

# Checks Minimum Supported Rust Version (MSRV)
msrv:
  cargo msrv verify
