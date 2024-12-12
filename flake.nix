{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        buildTarget = "x86_64-unknown-linux-gnu";
        packageName = "average_color";

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ buildTarget ];
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        packages.default = rustPlatform.buildRustPackage {
          name = packageName;
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          buildPhase = ''
            cargo build --release -p ${packageName} --target=${buildTarget}
          '';

          installPhase = ''
            mkdir -p $out/lib
            cp target/${buildTarget}/release/*.wasm $out/lib/
          '';

          # Disable checks if they only work for WASM
          # doCheck = false;
        };
      });
}
