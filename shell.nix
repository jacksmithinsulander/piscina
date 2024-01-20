{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell
{
  nativeBuildInputs = with pkgs; [
    cargo
    rustup
    mysql80
  ];
  shellHook = ''
    echo "Godbye World"
  '';
}