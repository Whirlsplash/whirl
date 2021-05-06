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

#      fromImage = pkgs.dockerTools.buildImage {
#        name = "bash";
#        tag = "latest";
#        contents = pkgs.bashInteractive;
#      };

      contents = [ pkg ];

      config = {
        Cmd = [ "/bin/whirl" "run" ];
        WorkingDir = "/";
        Env = [ "DATABASE_URl=whirl.sqlite3" "DISABLE_PROMPT=true" ];
      };
    };

in dockerImage whirl
