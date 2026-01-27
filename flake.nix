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
              frontend-wasm
            ];
            preBuild = ''
              ln -s ${frontend-wasm}/lib server/pkg
            '';
          };
          frontend-wasm = naersk-lib.buildPackage {
            pname = "frontend";
            src = ./.;
            buildInputs = with pkgs; [ wasm-bindgen-cli_0_2_108 ];
            cargoBuildOptions =
              x:
              x
              ++ [
                "-p"
                "frontend"
              ];
            copyLibs = true;
            postInstall = ''
              mkdir -p $out/lib
              wasm-bindgen --target web --out-dir $out/lib $out/lib/frontend.wasm
            '';
          };
          default = self.packages.${system}.server;
        };
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              rustToolchain
              diesel-cli
              lazysql
              sqlite
              pkg-config
              openssl
              nil
              bun
              wasm-bindgen-cli_0_2_108
              just
              cargo-machete
              watchexec
            ];
          };
      }
    )
    // {
      nixosModules = {
        interlinked = import ./nix/service.nix { interlinked = self.packages; };
      };
    };
}
