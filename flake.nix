{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        packages = rec {
          default = interlinked;
          interlinked = naersk-lib.buildPackage {
            src = ./.;
            buildInputs = with pkgs; [ sqlite ];
          };
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
            wasm-bindgen-cli
            just
            opencode
          ];
        };
      }
    );
}
