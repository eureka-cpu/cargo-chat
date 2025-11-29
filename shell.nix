let
  inherit (builtins)
    currentSystem
    readFile
    fromJSON
    attrValues
    ;

  sources = fromJSON (readFile ./yae.json);
  fromSource = name: fetchTarball {
    inherit (sources.${name}) url sha256;
  };
in
{ system ? currentSystem
, pkgs ? import (fromSource "nixpkgs") {
    inherit system;
    overlays = [
      (import "${fromSource "fenix"}/overlay.nix")
      (import "${fromSource "yae"}/overlay.nix")
    ];
  }
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    yae
    # TODO: Fix this
    fenix.stable.toolchain
    rust-analyzer-nightly
    sqlx-cli
    sqlite
  ];
}
