{
  description = "Flake for Project Ra Lex Jin";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in  {
      devShell = with pkgs; mkShell rec {
        nativeBuildInputs = [
          pkg-config
          ldtk
          (pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
        ];

        buildInputs = [
          udev
          alsa-lib vulkan-loader
          # X11
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          # Wayland
          libxkbcommon
          wayland
        ];

        LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    }
  );
}