{
  description = "Zapnote: Lightning-fast template-based note generator ";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, }:

    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            bacon
            cargo
            cargo-msrv
            clippy
            just
            nixfmt
            rust-analyzer
            rustc
            rustfmt
          ];
        };
      });
}
