{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  whirl = import ./whirl.nix { inherit sources pkgs; };

  name = "Whirlsplash/whirl";
  tag = "latest";

in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ whirl ];

  config = {
    Cmd = [ "/bin/whirl" ];
    Env = [ ];
    WorkingDir = "/";
  };
}
