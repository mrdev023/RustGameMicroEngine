{
  description = "DRM test rust configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        rustPlatform = pkgs.recurseIntoAttrs (pkgs.makeRustPlatform {
          rustc = rust;
          cargo = rust;
        });

        libs = with pkgs; [ ]
          ++ pkgs.lib.optional pkgs.stdenv.hostPlatform.isDarwin [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              (rust.override { extensions = ["rust-src" "rust-analyzer"]; })
              pkg-config
            ];

            buildInputs = libs;
          };
        };

        packages = {
          default = rustPlatform.buildRustPackage {
            pname = "rust_micro_game_engine";
            version = "0.1.0";

            src = self;

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = libs;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };
      });
}
