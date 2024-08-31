let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/5de1564aed415bf9d0f281461babc2d101dd49ff.tar.gz") {
    overlays = [ moz_overlay ];
  };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    jq
    sea-orm-cli

    (rustChannelOf {
      rustToolchain = ./server/rust-toolchain.toml;
    }).rust

    (rustChannelOf {
      version = "1.80.0";
      channel = "stable";
    }).rust
  ];
}
