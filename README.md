# Literate BitTorrent Client

A fully functional BitTorrent client implemented as a **literate program** using [noweb](https://www.cs.tufts.edu/~nr/noweb/). The source is a collection of `.nw` files that weave into comprehensive PDF documentation and tangle into a working Rust application.

## Overview

This project serves two purposes:

1. **Educational**: The document reads as a tutorial on BitTorrent internals, suitable for someone learning the protocol from scratch.
2. **Practical**: The tangled code downloads torrents from public trackers and swarms. No toy subset - real interoperability.

The code organization follows narrative logic rather than module boundaries. Concepts build on each other, making it ideal for understanding how BitTorrent actually works.

## Documentation

The complete literate documentation is available as a PDF:

**[doc/literate-bittorrent.pdf](doc/literate-bittorrent.pdf)**

This document covers:
- Bencode encoding/decoding
- Torrent file (metainfo) parsing
- HTTP tracker protocol
- Peer wire protocol messages
- Piece and block management
- Download strategy (rarest-first, endgame mode)
- Disk I/O with sparse files
- The complete client orchestration

## Quick Start

### Prerequisites

This project uses [Nix](https://nixos.org/) for reproducible development. All dependencies (Rust, noweb, TeX Live, etc.) are managed through `shell.nix`.

```bash
# Enter the development environment
nix-shell

# Build the client
just build

# Download a torrent
just run path/to/file.torrent
```

### Common Commands

All commands are run inside `nix-shell`:

| Command | Description |
|---------|-------------|
| `just build` | Compile the Rust project |
| `just test` | Run all tests |
| `just lint` | Run clippy linter |
| `just weave` | Generate PDF documentation |
| `just tangle` | Extract Rust source from noweb files |
| `just run <file.torrent>` | Download a torrent |
| `just demo-ubuntu` | Download Ubuntu ISO (demo) |
| `just e2e` | Run end-to-end integration tests |
| `just ci` | Full CI check (build + test + lint + fmt) |
| `just clean` | Remove all generated artifacts |

### Downloading a Torrent

```bash
nix-shell
just build

# Basic usage
./target/debug/literate-bittorrent path/to/file.torrent

# With options
./target/debug/literate-bittorrent file.torrent -o ./downloads -v

# Or use the just recipe
just run path/to/file.torrent
```

### CLI Options

```
Usage: literate-bittorrent [OPTIONS] <TORRENT>

Arguments:
  <TORRENT>  Path to the .torrent file

Options:
  -o, --output <DIR>              Output directory [default: .]
  -p, --port <PORT>               Listen port [default: 6881]
  -v, --verbose                   Enable verbose logging
      --max-download-rate <RATE>  Maximum download rate (e.g., 1M, 500K)
      --max-upload-rate <RATE>    Maximum upload rate (e.g., 1M, 500K)
      --json-log                  Output logs in JSON format
  -h, --help                      Print help
  -V, --version                   Print version
```

## Project Structure

```
literate-bittorrent/
├── nw/                    # Literate source (the actual source of truth)
│   ├── 00-main.nw         # Master file, intro, main program
│   ├── 01-bencode.nw      # Bencode encoding/decoding
│   ├── 02-metainfo.nw     # .torrent file parsing
│   ├── 03-tracker.nw      # HTTP tracker protocol
│   ├── 04-messages.nw     # Peer wire protocol messages
│   ├── 05-pieces.nw       # Piece/block management
│   ├── 06-cli.nw          # CLI argument parsing
│   ├── 07-client.nw       # Client orchestration
│   ├── 08-disk.nw         # Disk I/O
│   └── 09-appendix.nw     # BEP references
├── src/                   # Generated Rust code (do not edit directly)
├── doc/                   # Generated PDF documentation
├── tests/                 # Integration tests
│   └── fixtures/          # Test torrent files
├── shell.nix              # Nix development environment
├── justfile               # Task runner recipes
├── Cargo.toml             # Rust dependencies
└── prd.md                 # Product requirements document
```

**Important**: The `nw/` directory is the source of truth. All `.rs` files in `src/` are generated via `just tangle` and should not be edited directly.

## Features

- Tracker-based peer discovery (HTTP)
- Compact and dictionary peer list formats (BEP 3, BEP 23)
- Piece verification with SHA1
- Rarest-first piece selection
- Request pipelining (concurrent block requests)
- Endgame mode for fast completion
- Resume support (verifies existing pieces on startup)
- Bandwidth limiting (upload/download rate caps)
- Multi-tracker support (announce-list)
- Graceful shutdown with tracker notification
- JSON logging mode for monitoring

## Non-Goals

This is an educational implementation. The following are intentionally not supported:

- DHT (Kademlia) - tracker-based discovery only
- uTP (LEDBAT) transport - TCP only
- Seeding after completion
- Magnet links
- GUI/TUI - CLI only

## Development

### Literate Programming Workflow

1. Edit `.nw` files in `nw/`
2. Run `just tangle` to regenerate Rust source
3. Run `just build` to compile
4. Run `just weave` to regenerate PDF documentation

### Watch Mode

For active development, use watch mode to automatically re-tangle on changes:

```bash
just watch
```

### Running Tests

```bash
# Unit tests
just test

# Integration tests (requires network)
just e2e
```

## References

- [BEP 3: The BitTorrent Protocol Specification](http://www.bittorrent.org/beps/bep_0003.html)
- [BEP 23: Tracker Returns Compact Peer Lists](http://www.bittorrent.org/beps/bep_0023.html)
- [Unofficial BitTorrent Specification](https://wiki.theory.org/BitTorrentSpecification)
- [noweb documentation](https://www.cs.tufts.edu/~nr/noweb/)

## License

MIT
