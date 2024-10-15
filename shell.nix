let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/5de1564aed415bf9d0f281461babc2d101dd49ff.tar.gz") {
    overlays = [ moz_overlay ];
  };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    #next.js
    nodejs_22
    corepack_22
    #backend
    sea-orm-cli
    cargo-watch
    jq
    httpie
    #docker
    docker
    docker-compose

    (rustChannelOf {
      rustToolchain = ./backend/server/rust-toolchain.toml;
    }).rust

    (rustChannelOf {
      version = "1.81.0";
      channel = "stable";
    }).rust
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
