{
  description = "Flake for ProjectRaLexJin";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }: {
    devShell = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      with pkgs;
      mkShell {
        buildInputs = [
          ldtk
        ];
      }
    );
  };
}