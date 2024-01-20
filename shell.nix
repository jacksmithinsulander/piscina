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
    mysql -u root -e "CREATE DATABASE liquidity_pools;"
    mysql -u root liquidity_pools < ./data/init.sql
    mysqlshow -u root
  '';
}