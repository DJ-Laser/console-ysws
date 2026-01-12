{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  } @ inputs: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {inherit system overlays;};
    lib = pkgs.lib;

    rustToolchain = pkgs.pkgsBuildHost.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = ["rust-src"];
      targets = ["wasm32-unknown-emscripten"];
    });

  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [rustToolchain emscripten];
        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    };
  };
}
