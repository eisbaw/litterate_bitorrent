# Justfile for literate BitTorrent client
# All recipes assume execution within nix-shell

# Default recipe: show available recipes
default:
    @just --list

# ============================================================================
# Core Literate Programming Recipes
# ============================================================================

# Tangle: extract Rust code from noweb source
# Creates src/ directory and generates all Rust source files from nw/*.nw
tangle:
    @mkdir -p src
    notangle -Rmain.rs nw/00-main.nw > src/main.rs
    notangle -Rlib.rs nw/00-main.nw > src/lib.rs
    notangle -Rbencode.rs nw/01-bencode.nw > src/bencode.rs
    notangle -Rmetainfo.rs nw/02-metainfo.nw > src/metainfo.rs
    notangle -Rtracker.rs nw/03-tracker.nw > src/tracker.rs
    notangle -Rmessages.rs nw/04-messages.nw > src/messages.rs
    notangle -Rpieces.rs nw/05-pieces.nw > src/pieces.rs
    notangle -Rcli.rs nw/06-cli.nw > src/cli.rs
    notangle -Rclient.rs nw/07-client.nw > src/client.rs
    notangle -Rdisk.rs nw/08-disk.nw > src/disk.rs

# Weave: generate PDF documentation from noweb source
# Creates doc/ directory and produces doc/literate-bittorrent.pdf
# Uses XeLaTeX for fontspec support and compatibility with noweb.sty
# Processes all .nw files in order: main, chapters 1-8, appendix
# Note: -index is omitted for multi-file processing compatibility
weave:
    @mkdir -p doc
    noweave -delay nw/00-main.nw nw/01-bencode.nw nw/02-metainfo.nw nw/03-tracker.nw nw/04-messages.nw nw/05-pieces.nw nw/06-cli.nw nw/07-client.nw nw/08-disk.nw nw/09-appendix.nw > doc/literate-bittorrent.tex
    cd doc && xelatex -shell-escape literate-bittorrent.tex
    cd doc && xelatex -shell-escape literate-bittorrent.tex

# Weave fast: single-pass draft mode (no index, faster)
weave-fast:
    @mkdir -p doc
    noweave -delay nw/00-main.nw nw/01-bencode.nw nw/02-metainfo.nw nw/03-tracker.nw nw/04-messages.nw nw/05-pieces.nw nw/06-cli.nw nw/07-client.nw nw/08-disk.nw nw/09-appendix.nw > doc/literate-bittorrent.tex
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

# Lint: run clippy for linting (all targets including tests)
lint: tangle
    cargo clippy --all-targets -- -D warnings

# Format check: verify code formatting
fmt:
    cargo fmt --check

# Format fix: apply formatting corrections
fmt-fix:
    cargo fmt

# End-to-end tests: run integration tests that require network access
# These tests are marked #[ignore] in the source and require explicit invocation
# Note: Requires network access to contact real BitTorrent trackers
# Uses --test to run only integration tests (not unit tests or doc-tests)
e2e: tangle
    cargo test --test '*' -- --ignored --nocapture

# Peer handshake test: test peer connection flow (connect, handshake, bitfield, interested, unchoke)
# Requires network access to contact real BitTorrent peers
test-peer-handshake: tangle
    cargo test --test peer_integration -- --ignored --nocapture

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

# Download: run the client with ubuntu torrent for manual testing
# Creates a ./downloads directory for output
# Usage: just download
download: tangle build
    @mkdir -p downloads
    cargo run -- tests/fixtures/ubuntu.torrent -o ./downloads -v

# Download with custom output directory
# Usage: just download-to /path/to/output
download-to output: tangle build
    @mkdir -p {{output}}
    cargo run -- tests/fixtures/ubuntu.torrent -o {{output}} -v
