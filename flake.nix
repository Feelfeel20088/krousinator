{
  description = "Krousinator dev shell";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-24.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-bin = pkgs.rust-bin;
      in
      {
        formatter = pkgs.nixpkgs-fmt;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl

            (rust-bin.stable.latest.default.override {
              extensions = [
                "clippy"
                "rust-src"
                "rust-analyzer"
              ];
              targets = [
                "thumbv6m-none-eabi"
                "x86_64-unknown-linux-gnu"
              ];
            })
          ];
        };
      }
    );
}
