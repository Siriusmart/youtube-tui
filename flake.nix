{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
        rustVersion = pkgs.rust-bin.stable.latest.default;
        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        youtube-tui = rustPlatform.buildRustPackage {
          pname =
            toml.package.name; # make this what ever your cargo.toml package.name is
          version = "v${toml.package.version}";
          src = ./.; # the folder with the cargo.toml

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
            python3
            makeWrapper
          ];

          buildInputs = with pkgs; [
            openssl
            xorg.libxcb
            mpv
            libsixel
          ];

          postInstall = ''
            wrapProgram $out/bin/youtube-tui \
              --prefix PATH : ${nixpkgs.lib.makeBinPath [ pkgs.mpv ]}
          '';
        };
      in {
        defaultPackage = youtube-tui;
        devShell = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override {extensions = ["rust-src"];})
            pkgs.pkg-config
            pkgs.python3
            pkgs.openssl
            pkgs.mpv
            pkgs.libsixel
            pkgs.xorg.libxcb
          ];
        };
      }
    );
}
