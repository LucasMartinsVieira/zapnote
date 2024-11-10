{
  description = "Zapnote: Lightning-fast template-based note generator ";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:

    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        naersk' = pkgs.callPackage naersk { };
      in {
        # For `nix develop`:
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

        packages = {
          # For `nix build` `nix run`, & `nix profile install`:
          default = naersk'.buildPackage {
            pname = "zapnote";
            version = "git";

            src = ./.;
            doCheck = true; # run `cargo test` on build

            meta = with pkgs.lib; {
              description = "Lightning-fast template-based note generator";
              homepage = "https://github.com/lucasmartinsvieira/zapnote";
              license = licenses.mit;
              # maintainers = with maintainers; [ ];
              mainProgram = "zn";
            };
          };
        };
      });
}
