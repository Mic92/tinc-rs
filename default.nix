with import <nixpkgs> {};
mkShell {
  packages = [
    bashInteractive
    cargo
    rustc
    cargo-watch
  ];
}
