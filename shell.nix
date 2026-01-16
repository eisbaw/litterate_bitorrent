# Reproducible development environment for literate BitTorrent client
# Pinned nixpkgs for reproducibility

let
  # Pin nixpkgs to a specific commit for reproducibility
  # Using nixos-unstable branch (January 2025) for Rust 1.83+
  nixpkgs = fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/a9e2a5b6c8996c1863bacf03b91de9b2e6b90914.tar.gz";
    sha256 = "0022f3d7bmxjx7sjc69fp8wx380m1bz6f081yhf8hg8iszhgh631";
  };

  pkgs = import nixpkgs { };

  # Font configuration for XeLaTeX/LuaLaTeX to find fonts
  fontsConf = pkgs.makeFontsConf {
    fontDirectories = [ pkgs.jetbrains-mono pkgs.lmodern ];
  };

  # TeX Live with minted support
  texlive = pkgs.texlive.combine {
    inherit (pkgs.texlive)
      scheme-medium
      # LuaLaTeX requirements
      luatex
      luatexbase
      # minted for code highlighting
      minted
      fvextra
      upquote
      lineno
      catchfile
      xstring
      framed
      float
      # Font support
      fontspec
      # Additional useful packages
      etoolbox
      xcolor
      fancyvrb
      ifplatform
      ;
  };

in pkgs.mkShell {
  buildInputs = [
    # Rust toolchain (stable)
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.rust-analyzer

    # Build dependencies for reqwest/TLS
    pkgs.pkg-config
    pkgs.openssl

    # Literate programming
    pkgs.noweb

    # Document generation
    texlive

    # Fonts for PDF generation
    pkgs.jetbrains-mono
    pkgs.lmodern

    # Pygments for minted code highlighting
    pkgs.python3Packages.pygments

    # which is needed by minted to find pygmentize
    pkgs.which

    # Task runner
    pkgs.just

    # File watching
    pkgs.watchexec
  ];

  # Minimal shellHook - no verbose output
  shellHook = ''
    export CARGO_HOME="$PWD/.cargo"
    export PATH="$CARGO_HOME/bin:$PATH"
    export FONTCONFIG_FILE=${fontsConf}
    # OSFONTDIR for luaotfload to find system fonts (multiple paths separated by colon)
    export OSFONTDIR="${pkgs.jetbrains-mono}/share/fonts:${pkgs.lmodern}/share/fonts"
    # noweb.sty is in the tex output of the noweb package
    export TEXINPUTS="${pkgs.noweb.tex}/tex/latex/noweb:$TEXINPUTS"
  '';
}
