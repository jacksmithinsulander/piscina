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
    cargo install cargo-run-script
    cargo build
    mysql -u root -e "CREATE DATABASE liquidity_pools;"
    mysql -u root liquidity_pools < ./data/init.sql
    mysqlshow -u root liquidity_pools found_pools
    mysql -u root -e "USE liquidity_pools; DESCRIBE found_pools;"
  '';
}