{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        rustNightly = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" "llvm-tools-preview" ];
        };

        naersk-lib = pkgs.callPackage naersk {
          cargo = rustNightly;
          rustc = rustNightly;
        };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        
        devShell = pkgs.mkShell {
          buildInputs = [ 
            rustNightly 
            pkgs.pre-commit 
          ];
          
          RUST_SRC_PATH = "${rustNightly}/lib/rustlib/src/rust/library";
        };
      }
    );
}
