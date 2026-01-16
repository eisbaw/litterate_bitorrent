# Reproducible development environment for literate BitTorrent client
# Pinned nixpkgs for reproducibility

let
  # Pin nixpkgs to a specific commit for reproducibility
  # Using nixos-24.11 branch (stable as of 2025)
  nixpkgs = fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/d3c42f187194c26d9f0309a8ecc469d6c878ce33.tar.gz";
    sha256 = "0bmnxsn9r4qfslg4mahsl9y9719ykifbazpxxn1fqf47zbbanxkh";
  };

  pkgs = import nixpkgs { };

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
  '';
}
