{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShell =
          with pkgs;
          mkShell.override
            {
              stdenv = stdenvAdapters.useMoldLinker clangStdenv;
            }
            {
              nativeBuildInputs = [
                rust-bin.nightly."2024-06-03".default
                cargo-make
                openssl
                pkg-config
                cargo-watch
              ];

              RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
      }
    );
}
