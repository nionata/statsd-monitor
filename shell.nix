let
  pkgs = import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; };
  rust-version = "1.77.2";
  rust = pkgs.rust-bin.stable.${rust-version}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };
in pkgs.mkShell { buildInputs = [ rust ]; }
