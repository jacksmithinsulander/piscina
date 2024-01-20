{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell
{
  nativeBuildInputs = with pkgs; [
    cargo
    rustup
    mysql80
    curl
  ];
  shellHook = ''
    echo "Godbye World"
    curl wttr.in
  '';
}