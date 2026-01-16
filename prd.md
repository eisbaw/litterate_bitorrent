# Literate BitTorrent Client

## Product Requirements Document

### Overview

A fully functional BitTorrent client implemented as a literate program using noweb. The source is a collection of `.nw` files in `nw/` that weave into comprehensive PDF documentation and tangle into a working Rust application. The goal is both pedagogical—teaching the BitTorrent protocol through narrative—and practical: a client capable of downloading real torrents.

### Goals

1. **Educational clarity**: The document reads as a tutorial on BitTorrent internals, suitable for someone learning the protocol from scratch.

2. **Working implementation**: The tangled code downloads torrents from public trackers and swarms. No toy subset—real interoperability.

3. **Literate structure**: Code organization follows narrative logic, not module convenience. Concepts build on each other.

4. **Modern Rust**: Strong typing, async/await with tokio, zero-copy parsing where sensible, no unsafe unless justified.

### Non-Goals

- DHT (Kademlia) support—tracker-based discovery only
- uTP (LEDBAT) transport—TCP only
- Seeding after completion
- Magnet link resolution
- GUI or TUI—CLI only
- Multi-file torrent optimization (will work, but no per-file priority)

### Target Audience

- Developers curious about protocol implementation
- Students learning networking and file formats
- Anyone interested in literate programming as methodology

### Technical Constraints

| Constraint | Choice |
|------------|--------|
| Language | Rust (stable) |
| Build system | Cargo |
| Dev environment | shell.nix |
| Task runner | just |
| Literate tool | noweb |
| TeX engine | LuaLaTeX (via TeX Live) |
| Document fonts | fontspec + modern monospace (e.g., JetBrains Mono) |
| Code highlighting | minted + Pygments |
| Async runtime | tokio |
| Crypto | sha1 crate (or ring) |

### Project Structure

```
literate-bittorrent/
├── shell.nix
├── justfile
├── PRD.md
│
├── doc/                          # Woven output (generated, .gitignore'd)
│   └── literate-bittorrent.pdf
│
├── src/                          # Tangled output (generated, .gitignore'd)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── bencode.rs
│       ├── metainfo.rs
│       ├── tracker.rs
│       ├── messages.rs
│       ├── peer.rs
│       ├── pieces.rs
│       ├── strategy.rs
│       └── client.rs
│
├── nw/                           # Literate source (the actual source of truth)
│   ├── 00-main.nw                # Master file, includes others, intro prose
│   ├── 01-bencode.nw             # Bencode encoding/decoding
│   ├── 02-metainfo.nw            # .torrent file parsing
│   ├── 03-tracker.nw             # HTTP tracker protocol
│   ├── 04-messages.nw            # Peer wire protocol message types
│   ├── 05-peer.nw                # Peer connection handling
│   ├── 06-pieces.nw              # Piece/block management, disk I/O
│   ├── 07-strategy.nw            # Piece selection, request pipelining
│   ├── 08-client.nw              # Orchestration, main event loop
│   └── 09-cli.nw                 # CLI entry point
│
└── tests/                        # Not literate—verification code
    ├── bencode_tests.rs
    ├── metainfo_tests.rs
    └── fixtures/
        └── ubuntu.torrent        # Real torrent for integration tests
```

The `nw/` directory is the source of truth. All `.rs` files and `Cargo.toml` are generated via `notangle` and should not be edited directly. The `doc/` and `src/` directories are in `.gitignore`.

### External Dependencies (Cargo crates, minimal)

- `tokio` — async runtime
- `reqwest` — async HTTP for tracker announces
- `sha1` — SHA1 hashing for info hash and piece verification
- `clap` — CLI argument parsing
- `bytes` — efficient byte buffer manipulation
- `thiserror` — ergonomic error types

No BitTorrent protocol crates. No abstraction layers. Direct protocol implementation.

### Document Structure

The `.nw` files follow this narrative arc:

```
1. Introduction
   - What is BitTorrent?
   - Protocol overview
   - Document roadmap

2. Bencode
   - Grammar definition
   - Decoder implementation
   - Encoder implementation
   - Round-trip property

3. Metainfo Files (.torrent)
   - Structure and fields
   - Info hash computation
   - Parsing implementation

4. Tracker Protocol
   - Announce URL construction
   - HTTP tracker request/response
   - Peer list extraction
   - Announce intervals and re-announces

5. Peer Wire Protocol
   - TCP connection setup
   - Handshake format and validation
   - Message framing (length prefix)
   - Message types:
     - choke / unchoke
     - interested / not interested
     - have / bitfield
     - request / piece / cancel

6. Piece Management
   - Piece and block geometry
   - SHA1 verification
   - Disk I/O and file mapping
   - Progress tracking

7. Download Strategy
   - Peer selection
   - Piece selection (rarest first)
   - Request pipelining
   - Endgame mode

8. Orchestration
   - Main event loop
   - Peer connection management
   - Choking algorithm (simplified)
   - Completion detection

9. CLI Interface
   - Argument parsing
   - Progress display
   - Graceful shutdown

10. Appendices
    - Protocol references (BEP links)
    - Test vectors
    - Future work (DHT, PEX, etc.)
```

### File Outputs (from tangle)

```
src/
  Cargo.toml
  src/
    main.rs
    lib.rs
    bencode.rs
    metainfo.rs
    tracker.rs
    messages.rs
    peer.rs
    pieces.rs
    strategy.rs
    client.rs
```

Each module corresponds to a narrative chapter, but chunk ordering within the `.nw` allows forward references and pedagogical sequencing.

### Development Workflow

```bash
# Enter dev environment
nix-shell

# Tangle code from literate source
just tangle

# Build
just build

# Run tests
just test

# Weave documentation (LuaLaTeX)
just weave

# Download a torrent
just run path/to/file.torrent

# Full rebuild (tangle + build + weave)
just all
```

### Testing Strategy

- **Unit tests**: Bencode round-trips, metainfo parsing against known `.torrent` files, message serialization (in-module `#[cfg(test)]`)
- **Integration tests**: Download a small, well-seeded public domain torrent (e.g., a Linux ISO)
- **Property tests**: Bencode encode/decode inverse property with `proptest` or `quickcheck`

Integration tests live in a separate `tests/` directory (not literate—they're verification, not narrative).

### Success Criteria

1. Can download a real torrent (e.g., Ubuntu ISO) to completion
2. Documentation builds to PDF via LuaLaTeX without errors
3. Code compiles with no warnings (`cargo build` + `cargo clippy`)
4. A reader unfamiliar with BitTorrent can follow the document and understand the protocol

### Milestones

| Phase | Deliverable | ~Lines |
|-------|-------------|--------|
| 1 | Bencode + metainfo parsing | 400 |
| 2 | Tracker announces | 300 |
| 3 | Peer handshake + messages | 800 |
| 4 | Piece management + disk I/O | 700 |
| 5 | Download strategy | 600 |
| 6 | Orchestration + event loop | 1000 |
| 7 | CLI + polish | 400 |
| 8 | Documentation prose | — |

Total code: ~4200 lines, leaving headroom for edge cases and error handling.

### References

- [BEP 3: The BitTorrent Protocol Specification](http://www.bittorrent.org/beps/bep_0003.html)
- [BEP 23: Tracker Returns Compact Peer Lists](http://www.bittorrent.org/beps/bep_0023.html)
- [Unofficial BitTorrent Specification (wiki.theory.org archive)](https://wiki.theory.org/BitTorrentSpecification)
- [Knuth, Literate Programming (1984)](http://www.literateprogramming.com/knuthweb.pdf)
- [noweb documentation](https://www.cs.tufts.edu/~nr/noweb/)

### License

MIT for code and documentation. This is meant to be studied and adapted.
