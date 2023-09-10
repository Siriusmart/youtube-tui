{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            toml.package.name; # make this what ever your cargo.toml package.name is
          version = "v${toml.package.version}";
          src = ./.; # the folder with the cargo.toml

          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = with pkgs; [
            openssl
            xorg.libxcb
            libsixel
            mpv
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            python3
          ];
        };

      in
      {
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}

