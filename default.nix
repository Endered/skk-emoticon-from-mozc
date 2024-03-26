{
  pkgs ? import <nixpkgs> {}
}:
let
  mozc-dictionary = pkgs.fetchurl {
    url = "https://github.com/google/mozc/blob/d3816084be54223f69308b9ede394a02e26753b8/src/data/emoticon/emoticon.tsv";
    hash = "sha256-NmVYs4C+8H3aJoIskQDR76vuU595YbLGNj1GFMSnYsQ=";
  };
in
pkgs.stdenv.mkDerivation {
  name = "skk-emoticon-from-mozc";

  src = ./.;

  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    rust-script
  ];

  buildPhase = ''
    export XDG_CACHE_HOME=$PWD/.cache
    cat ${mozc-dictionary} \
      | rust-script mozc-emoticon-converter.rs \
      > skk-emoticon.utf8
  '';

  installPhase = ''
    mkdir -p $out/share
    cp skk-emoticon.utf8 $out/share/skk-emoticon.utf8
  '';
}
