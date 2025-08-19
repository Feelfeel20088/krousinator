{
    rustPlatform,
    pkgs
}:

{

    krousinator = pkgs.rustPlatform.buildRustPackage {
        name = "Krousinator";
        src = ../.;
        buildInputs = with pkgs; [ openssl ];
        nativeBuildInputs = with pkgs; [ pkgs.pkg-config ];
        cargoLock.lockFile = ../Cargo.lock;
        cargoBuildFlags = [ "-p" "krousinator" ];
        env.OPENSSL_NO_VENDOR = "1";
    };

    kroushive = pkgs.rustPlatform.buildRustPackage {
        name = "Kroushive";
        src = ../.;
        buildInputs = with pkgs; [ openssl ];
        nativeBuildInputs = with pkgs; [ pkgs.pkg-config ];
        cargoLock.lockFile = ../Cargo.lock;
        cargoBuildFlags = [ "-p" "kroushive" ];
        env.OPENSSL_NO_VENDOR = "1";
    };

}