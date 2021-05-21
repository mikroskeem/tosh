{ pkgs ? import <nixpkgs> { }, ... }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs;
    [
      cargo
      rustfmt
    ];
}
