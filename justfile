default:
  @just --list

test:
  cargo test

fmt:
  cargo fmt --all
  nixfmt -- *.nix

lint:
  cargo fmt --all --check
  cargo clippy --all-features --all-targets -- -Dwarnings
  cargo msrv verify
  nixfmt --check -- *.nix
