{
  description = "Rust OS Kernel Development Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust = pkgs.rust-bin.stable.latest.default;

      buildInputs = with pkgs; [
        rust
        rust-analyzer
        clippy
        # pkgs.gcc
        pkg-config
        # pkgs.SDL2
        xorg.libXext
        xorg.libX11
        xorg.libXrandr
        xorg.libXcursor
        xorg.libXScrnSaver

        #things from iced
        expat
        fontconfig
        freetype
        freetype.dev
        libGL
        # pkg-config
        # xorg.libX11
        # xorg.libXcursor
        xorg.libXi
        # xorg.libXrandr
        wayland
        libxkbcommon
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        inherit buildInputs;
        natvieBuildInputs = with pkgs; [
          pkg-config
        ];

        #from iced
        LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

        shellHook = ''
          exec zsh -c "nvim"
        '';
      };
    };
}
