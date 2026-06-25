{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
      rust-overlay,
      treefmt-nix,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        treefmtModule = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;
      in
      rec {
        formatter = treefmtModule.config.build.wrapper;

        checks = {
          formatting = treefmtModule.config.build.check self;
          server_x86-64_linux = packages.server;
          cli_x86-64_linux = packages.cli;
          frontend_x86-64_linux = packages.frontend;
        };

        packages = {
          cli = naersk-lib.buildPackage {
            pname = "cli";
            cargoBuildOptions =
              x:
              x
              ++ [
                "-p"
                "cli"
              ];
            src = ./.;
          };
          server = naersk-lib.buildPackage {
            pname = "server";
            cargoBuildOptions =
              x:
              x
              ++ [
                "-p"
                "server"
              ];
            src = ./.;
            buildInputs = with pkgs; [
              sqlite
            ];
          };
          frontend =
            let
              # Build deps with naersk to get vendored crate sources.
              # Multi-step (default) so passthru.builtDependencies is populated.
              depsBuild = naersk-lib.buildPackage {
                pname = "frontend-deps";
                src = ./.;
                cargoBuildOptions =
                  x:
                  x
                  ++ [
                    "-p"
                    "frontend"
                    "--target"
                    "wasm32-unknown-unknown"
                  ];
                mode = "check";
                release = true;
              };
              depsDrv = builtins.head depsBuild.passthru.builtDependencies;
            in
            pkgs.runCommandLocal "frontend"
              {
                nativeBuildInputs = [
                  pkgs.trunk
                  pkgs.wasm-bindgen-cli_0_2_121
                  pkgs.binaryen
                  rustToolchain
                ];
              }
              ''
                cp -r ${./.} src
                chmod -R +w src
                cd src/frontend

                export CARGO_HOME=$PWD/../.cargo-home
                mkdir -p $CARGO_HOME
                cp ${depsDrv.cargoconfig} $CARGO_HOME/config.toml

                trunk build --offline --release --dist $out
              '';
          default = self.packages.${system}.server;
        };
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              rustToolchain
              diesel-cli
              trunk
              leptosfmt
              lazysql
              sqlite
              pkg-config
              openssl
              nil
              just
              cargo-machete
              watchexec
              act
            ];
          };
      }
    )
    // {
      nixosModules = {
        interlinked = import ./nix/service.nix { interlinked = self.packages; };
      };

      homeManagerModules = {
        interlinked = import ./nix/hm.nix { interlinked = self.packages; };
      };
    };
}
