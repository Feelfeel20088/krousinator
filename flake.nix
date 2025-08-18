{
  description = "Krousinator dev shell";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    # rust-overlay.url = "github:oxalica/rust-overlay";
    # rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo rustc rustfmt clippy rust-analyzer gnumake42
          ];

          nativeBuildInputs = [pkgs.pkg-config];
          env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
      # let
      #   overlays = [ (import rust-overlay) ];
      #   pkgs = import nixpkgs { inherit system overlays; };
      #   rust-bin = pkgs.rust-bin;
      # in
      # {
      #   formatter = pkgs.nixpkgs-fmt;

      #   devShell = pkgs.mkShell {
      #     buildInputs = with pkgs; [
      #       pkg-config
      #       openssl

      #       (rust-bin.stable.latest.default.override {
      #         extensions = [
      #           "clippy"
      #           "rust-src"
      #           "rust-analyzer"
      #         ];
      #         targets = [
      #           "thumbv6m-none-eabi"
      #           "x86_64-unknown-linux-gnu"
      #         ];
      #       })
      #     ];
      #   };
      # }
    );
}
