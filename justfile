# Justfile for literate BitTorrent client
# All recipes assume execution within nix-shell

# Default recipe: show available recipes
default:
    @just --list

# ============================================================================
# Core Literate Programming Recipes
# ============================================================================

# Tangle: extract Rust code from noweb source
# Creates src/ directory and generates src/main.rs from nw/00-main.nw
tangle:
    @mkdir -p src
    notangle -Rmain.rs nw/00-main.nw > src/main.rs

# Weave: generate PDF documentation from noweb source
# Creates doc/ directory and produces doc/literate-bittorrent.pdf
# Uses XeLaTeX for fontspec support and compatibility with noweb.sty
weave:
    @mkdir -p doc
    noweave -delay -index nw/00-main.nw > doc/literate-bittorrent.tex
    cd doc && xelatex -shell-escape literate-bittorrent.tex
    cd doc && xelatex -shell-escape literate-bittorrent.tex

# Weave fast: single-pass draft mode (no index, faster)
weave-fast:
    @mkdir -p doc
    noweave -delay nw/00-main.nw > doc/literate-bittorrent.tex
    cd doc && xelatex -shell-escape literate-bittorrent.tex

# ============================================================================
# Build Recipes
# ============================================================================

# Build: compile the Rust project (tangles first)
build: tangle
    cargo build

# Check: quick compile check without full build (tangles first)
check: tangle
    cargo check

# ============================================================================
# Testing and Quality Recipes
# ============================================================================

# Test: run all tests
test: tangle
    cargo test

# Lint: run clippy for linting
lint: tangle
    cargo clippy -- -D warnings

# Format check: verify code formatting
fmt:
    cargo fmt --check

# Format fix: apply formatting corrections
fmt-fix:
    cargo fmt

# End-to-end tests (placeholder for future implementation)
e2e:
    @echo "E2E tests not yet implemented"

# ============================================================================
# Workflow Recipes
# ============================================================================

# CI: full continuous integration check
ci: tangle build test lint fmt

# All: complete build including documentation
all: tangle build weave

# Watch: automatically re-tangle on .nw file changes
watch:
    watchexec -e nw -- just tangle

# ============================================================================
# Utility Recipes
# ============================================================================

# Clean: remove all generated artifacts
clean:
    rm -rf src/ doc/ target/

# View: open the generated PDF documentation
view:
    @if [ -f doc/literate-bittorrent.pdf ]; then \
        xdg-open doc/literate-bittorrent.pdf 2>/dev/null || \
        open doc/literate-bittorrent.pdf 2>/dev/null || \
        echo "Could not open PDF. File is at: doc/literate-bittorrent.pdf"; \
    else \
        echo "PDF not found. Run 'just weave' first."; \
    fi

# Run the client with a torrent file
run torrent:
    cargo run -- {{torrent}}
