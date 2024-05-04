{pkgs}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    (fenix.stable.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ])
  ];
}
