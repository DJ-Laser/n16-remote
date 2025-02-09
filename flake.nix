{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
  } @ inputs: let
    system = "x86_64-linux";
    overlays = [(fenix.overlays.default)];
    pkgs = import nixpkgs {inherit system overlays;};

    microcontrollerTarget = "thumbv6m-none-eabi";
    rustToolchain = with fenix.packages.${system};
      combine [
        default.cargo
        default.rustc
        default.rustfmt
        targets.${microcontrollerTarget}.latest.rust-std
      ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [alejandra rustToolchain elf2uf2-rs kicad];
      RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
    };
  };
}
