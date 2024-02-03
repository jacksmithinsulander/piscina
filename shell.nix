{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell
{
  nativeBuildInputs = with pkgs; [
    #cargo
    rustup
  ];
  shellHook = ''
    echo "Godbye World"
    cargo install cargo-run-script
    cargo build
  '';
}