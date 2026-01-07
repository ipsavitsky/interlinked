{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    bun2nix.url = "github:nix-community/bun2nix";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay, bun2nix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            bun2nix.overlays.default
          ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        packages = rec {
          cli = naersk-lib.buildPackage {
            pname = "cli";
            cargoBuildOptions = x: x ++ [ "-p" "cli" ];
            src = ./.;
          };
          server = naersk-lib.buildPackage {
            pname = "server";
            cargoBuildOptions = x: x ++ [ "-p" "server" ];
            src = ./.;
            buildInputs = with pkgs; [ sqlite ];
          };
          frontend-wasm = naersk-lib.buildPackage {
            pname = "frontend";
            src = ./.;
            buildInputs = with pkgs; [ wasm-bindgen-cli_0_2_100 ];
            cargoBuildOptions = x: x ++ [ "-p" "frontend" ];
            copyLibs = true;
            postInstall = ''
              mkdir -p $out/lib
              wasm-bindgen --target web --out-dir $out/lib $out/lib/frontend.wasm
            '';
          };
          frontend = pkgs.stdenv.mkDerivation {
            pname = "frontend";
            version = "0.0.1";
            src = ./.;
            nativeBuildInputs = [
              pkgs.bun2nix.hook
              frontend-wasm
            ];
            bunRoot = "frontend";
            bunDeps = pkgs.bun2nix.fetchBunDeps {
              bunNix = ./frontend/bun.nix;
            };
            buildPhase = ''
              ln -s ${frontend-wasm}/lib frontend/pkg
              cp -r frontend dist
            '';
            installPhase = ''
              mkdir -p $out/dist
              cp -R ./dist $out
            '';
          };
          default = self.packages.${system}.server;
        };
        devShell = with pkgs; mkShell {
          buildInputs = [
            rustToolchain
            diesel-cli
            lazysql
            sqlite
            pkg-config
            openssl
            nil
            bun
            wasm-bindgen-cli_0_2_100
            just
            cargo-machete
          ];
        };
      }
    );
}
