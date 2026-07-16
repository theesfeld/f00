# Nix flake for f00 — build from source (stub; refine as packaging matures).
{
  description = "f00 — modern ls clone in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "f00";
          version = "0.5.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # Binary package lives in the workspace package f00 (path crates/f00-cli).
          buildAndTestSubdir = "crates/f00-cli";
          meta = with pkgs.lib; {
            description = "Modern, friendly directory lister (ls rewrite)";
            homepage = "https://f00.sh";
            license = with licenses; [ mit asl20 ];
            mainProgram = "f00";
          };
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/f00";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [ rustc cargo rustfmt clippy ];
        };
      });
}
