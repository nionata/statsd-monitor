{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }: let
    pkgsForSystem = system:
      import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };
  in
    flake-utils.lib.eachDefaultSystem (system: rec {
      legacyPackages = pkgsForSystem system;
      devShells.default = import ./shell.nix {pkgs = legacyPackages;};
      formatter = legacyPackages.alejandra;
    });
}
