# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };


  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils, ...
  }: flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      #toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
    in
    {
        #rust fixes
      #openssl hack, likely just need OPENSSL_NO_VENDOR = 1 on build
      environment.variables = {
        PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
        OPENSSL_NO_VENDOR=1;
      };
      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.pkg-config
          pkgs.openssl
          pkgs.zlib
          #toolchain
        ];
      };
    }
  );
}