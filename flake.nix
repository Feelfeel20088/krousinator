{
  description = "Krousinator dev shell and package";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    # rust-overlay.url = "github:oxalica/rust-overlay";
    # rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, ... }: let 
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    krousinator = pkgs.callPackage ./nix/default.nix {};
  in {
    packages = {
      
      
      x86_64-linux = {
        kroushive = krousinator.kroushive;
      };

      
      x86_64-windows = {
        krousinator = krousinator.krousinator;
      };
    };

    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo rustc rustfmt clippy rust-analyzer gnumake42
      ];

      nativeBuildInputs = [pkgs.pkg-config];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}

# {
#   description = "Krousinator dev shell and package";

#   inputs = {
#     nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
#     crane.url = "github:ipetkov/crane";
#     flake-utils.url = "github:numtide/flake-utils";
#   };

#   outputs = { self, nixpkgs, crane, flake-utils, ... }:
#     flake-utils.lib.eachDefaultSystem (system:
#       let
#         pkgs = nixpkgs.legacyPackages.${system};
#         craneLib = crane.mkLib pkgs;
#       in
#     {
#       packages.default = craneLib.buildPackage {
#         src = craneLib.cleanCargoSource ./.;

#         # Add extra inputs here or any other derivation settings
#         doCheck = true;
#         buildInputs = [ pkgs.gnumake42 ];
#         nativeBuildInputs = [ pkgs.pkg-config ];

#       };
#       devShells."x86_64-linux".default = pkgs.mkShell {
#         buildInputs = with pkgs; [
#           cargo rustc rustfmt clippy rust-analyzer gnumake42
#         ];

#         nativeBuildInputs = [pkgs.pkg-config];
#           env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
#         };
#     });
# }
