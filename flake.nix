{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";

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
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
            sqlite
          ];
        };

        whirl = craneLib.buildPackage (
          commonArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          }
        );
      in
      {
        packages = {
          inherit whirl;

          default = whirl;

          docker = pkgs.dockerTools.buildLayeredImage {
            name = "fuwn/whirl";
            tag = "latest";

            config = {
              Entrypoint = [ "${whirl}/bin/whirl" ];

              Cmd = [
                "run"
                "distributor,hub"
              ];
            };
          };
        };

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
