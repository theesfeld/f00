# Nix flake for f00 — build from source.
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
          version = "0.11.0-dev";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # Workspace: build the lean CLI + separate TUI browser.
          cargoBuildFlags = [ "-p" "f00" "-p" "f00-tui" ];
          cargoTestFlags = [ "-p" "f00" "-p" "f00-tui" ];
          meta = with pkgs.lib; {
            description = "Modern, friendly directory lister (ls rewrite)";
            homepage = "https://f00.sh";
            license = with licenses; [ mit asl20 ];
            mainProgram = "f00";
          };
        };

        packages.f00-tui = self.packages.${system}.default;

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/f00";
        };

        apps.f00-tui = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/f00-tui";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [ rustc cargo rustfmt clippy ];
        };
      });
}
