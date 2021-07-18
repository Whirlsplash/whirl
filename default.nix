{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
with pkgs;

let
  rust = pkgs.callPackage ./nix/rust.nix { };

  srcNoTarget = dir:
    builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    dir;

  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  dhallpkgs = import sources.easy-dhall-nix { inherit pkgs; };
  src = srcNoTarget ./.;

  whirl = naersk.buildPackage {
    inherit src;
    doCheck = true;
    buildInputs = [ pkg-config openssl git ];
    remapPathPrefix = true;
  };

  config = stdenv.mkDerivation {
    pname = "whirl-config";
    version = "HEAD";
    buildInputs = [ dhallpkgs.dhall-simple ];

    #    phases = "installPhase";
    #
    #    installPhase = ''
    #    '';
  };

in pkgs.stdenv.mkDerivation {
  inherit (whirl) name;
  inherit src;
  phases = "installPhase";

  installPhase = ''
    mkdir -p $out $out/bin

    cp -rf ${whirl}/bin/whirl $out/bin/whirl
  '';
}
