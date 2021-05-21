{ pkgs ? import <nixpkgs> { }, ... }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs;
    [
      xxd
      openssl
      gen-oath-safe
    ];
}
