{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  callPackage = pkgs.lib.callPackageWith pkgs;
  whirl = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildLayeredImage {
      name = "Whirlsplash/whirl";
      tag = "latest";

      contents = [ pkg ];

      config = {
        Cmd = [ "/bin/whirl" ];
        WorkingDir = "/";
        Env = [ ];
      };
    };

in dockerImage whirl
