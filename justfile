default:
    @just --list

test:
  cargo test

fmt:
  cargo fmt --all

lint:
  cargo fmt --all --check
  cargo clippy --all-features --all-targets -- -Dwarnings
