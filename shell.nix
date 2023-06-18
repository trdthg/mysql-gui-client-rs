let
  # pkgs = import <nixpkgs> { };
  pkgs = import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # glib
    # glibc
    # libstdcxx5
    libiconv
    openssl_1_1
    openssl
    pkgconfig
    fontconfig

    # wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi

    libGL
  ];
  buildInputs = with pkgs;[
    # cargo
    systemd
  ];
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      pkgs.lib.makeLibraryPath  [
        # pkgs.stdenv.cc.cc.lib
        pkgs.libGL
        pkgs.openssl.dev
        # pkgs.glib
        # pkgs.glibc
        # pkgs.libstdcxx5
      ]
    }"'';
}
