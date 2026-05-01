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
      {
        formatter = treefmtModule.config.build.wrapper;

        checks = {
          formatting = treefmtModule.config.build.check self;
        };

        packages = rec {
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
