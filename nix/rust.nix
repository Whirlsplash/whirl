{ sources ? import ./sources.nix }:

let
    pkgs =
        import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
    channel = "nightly";
    date = "2021-02-20";
    targets = [ ];
    chan = pkgs.rustChannelOfTargets channel date targets;
in chan
