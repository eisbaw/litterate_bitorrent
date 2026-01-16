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

### Design Decisions

**Literariness vs Maintainability**: When these goals conflict, **educational clarity wins**. This is primarily a teaching tool. Code that is harder to maintain but easier to understand is preferred over clever abstractions.

**Generated Code Policy**: All tangled `.rs` files include a header comment indicating they are generated and should not be edited directly. The `nw/` directory remains the single source of truth.

**Hybrid Testing**: Tests live outside the literate source (`tests/`) because they are verification code, not narrative. However, key test vectors and expected behaviors are documented in the appendices to maintain prose/test alignment.

### Glossary

| Term | Definition |
|------|------------|
| **Swarm** | The collective set of peers sharing a specific torrent, identified by info hash |
| **Leecher** | A peer that does not have all pieces; still downloading |
| **Seeder** | A peer that has all pieces; only uploads (not implemented here) |
| **Peer ID** | 20-byte identifier for a client instance, sent in handshake and tracker announces |
| **Info Hash** | 20-byte SHA1 hash of the bencoded info dictionary; uniquely identifies a torrent |
| **Piece** | Fixed-size chunk of the torrent content (typically 256KB–4MB), SHA1-verified |
| **Block** | 16KB sub-unit of a piece; the unit of `request` messages (2^14 = 16384 bytes) |
| **Bitfield** | Bit array indicating which pieces a peer has; sent immediately after handshake |
| **Choking** | Flow control mechanism; a choked peer cannot request blocks from us |
| **Compact Peer Format** | BEP-23 format: 6 bytes per peer (4-byte IP + 2-byte port, big-endian) |
| **Keep-Alive** | Zero-length message sent to maintain connection when idle |

### Technical Constraints

| Constraint | Choice |
|------------|--------|
| Language | Rust (stable) |
| Build system | Cargo |
| Dev environment | shell.nix (pinned nixpkgs, explicit TeX packages) |
| Task runner | just |
| Literate tool | noweb |
| TeX engine | LuaLaTeX (via TeX Live) |
| Document fonts | fontspec + modern monospace (e.g., JetBrains Mono) |
| Code highlighting | minted + Pygments |
| Async runtime | tokio |
| Crypto | sha1 crate |

### Core Data Structures

Before implementation, these key data structures must be designed:

**Bencode Value Representation**
```rust
enum BencodeValue {
    Integer(i64),
    Bytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<Vec<u8>, BencodeValue>),
}
```
Using `BTreeMap` for dictionaries ensures canonical ordering for info hash computation.

**Torrent Metadata**
```rust
struct Metainfo {
    announce: String,
    info_hash: [u8; 20],
    piece_length: u32,
    piece_hashes: Vec<[u8; 20]>,
    total_length: u64,
    files: Vec<FileInfo>,  // single-file torrents have one entry
    name: String,
}
```

**Piece State**
```rust
enum PieceState {
    Missing,
    InProgress { blocks_received: BitVec },
    Verified,
    Failed,  // hash mismatch, will retry
}
```
For a torrent with 10,000 pieces, this is ~10KB of state plus block tracking for in-progress pieces.

**Peer Connection State**
```rust
struct PeerState {
    // Local state toward remote
    am_choking: bool,      // we are choking them (default: true)
    am_interested: bool,   // we want pieces from them (default: false)

    // Remote state toward us
    peer_choking: bool,    // they are choking us (default: true)
    peer_interested: bool, // they want pieces from us (default: false)

    // What they have
    bitfield: BitVec,

    // In-flight requests
    pending_requests: VecDeque<BlockRequest>,
}
```
Data flows only when: remote is **not choking us** AND we are **interested**.

**Block Request Tracking**
```rust
struct BlockRequest {
    piece_index: u32,
    offset: u32,
    length: u32,
    requested_at: Instant,
    peer: PeerId,
}
```
Bounded queue per peer (typically 10–16 concurrent requests for pipelining).

### Error Handling

**Error Taxonomy**
```rust
enum Error {
    // Bencode errors
    BencodeInvalidFormat(String),
    BencodeUnexpectedType { expected: &'static str, found: &'static str },

    // Metainfo errors
    MetainfoMissingField(&'static str),
    MetainfoInvalidPieceLength,

    // Tracker errors
    TrackerHttpError(reqwest::Error),
    TrackerErrorResponse(String),  // "failure reason" from tracker
    TrackerInvalidResponse(String),

    // Peer protocol errors
    PeerHandshakeFailed(String),
    PeerInvalidMessage,
    PeerTimeout,
    PeerDisconnected,

    // Piece errors
    PieceHashMismatch { index: u32 },

    // IO errors
    IoError(std::io::Error),
}
```

**Recovery Strategy**
- **Transient failures** (network timeout, peer disconnect): Log, retry with backoff, continue with other peers
- **Permanent failures** (bad hash, invalid metainfo): Fail fast, report clearly, do not retry
- **Graceful degradation**: Continue downloading with fewer peers if some fail

**Logging**: Use `tracing` crate. Log at:
- `INFO`: Piece completed, peer connected/disconnected, download progress
- `WARN`: Peer timeout, hash mismatch (will retry), tracker temporary failure
- `ERROR`: Fatal errors, invalid torrent file
- `DEBUG`: Individual block requests, message parsing

### Disk I/O Strategy

**Approach**: Explicit `seek` + `write` (not mmap) for pedagogical clarity.

**File Mapping**: Torrent content is a single logical byte stream. For multi-file torrents:
```
piece_start_byte = piece_index * piece_length
piece_end_byte = piece_start_byte + actual_piece_length

For each file:
  if piece overlaps file byte range:
    calculate file offset and length to write
    seek and write
```

**Pre-allocation**: Sparse files (no pre-allocation). OS handles actual disk allocation on write.

**Write Strategy**: Buffer complete pieces in memory, verify hash, then write atomically. No partial piece writes to disk.

### Timeouts

| Operation | Timeout | Notes |
|-----------|---------|-------|
| Tracker announce | 30 seconds | HTTP request timeout |
| Peer TCP connect | 10 seconds | Connection establishment |
| Peer handshake | 10 seconds | After TCP connected |
| Block request | 30 seconds | Per-request; triggers cancel + re-request from another peer |
| Keep-alive interval | 120 seconds | Send keep-alive if no messages sent |
| Idle peer disconnect | 180 seconds | Disconnect if no useful activity |

### Project Structure

```
literate-bittorrent/
├── shell.nix
├── justfile
├── Cargo.toml                    # NOT generated—maintained manually
├── Cargo.lock                    # NOT generated—committed for reproducibility
├── PRD.md
│
├── doc/                          # Woven output (generated, .gitignore'd)
│   └── literate-bittorrent.pdf
│
├── src/                          # Tangled output (generated, .gitignore'd)
│   ├── main.rs
│   ├── lib.rs
│   ├── bencode.rs
│   ├── metainfo.rs
│   ├── tracker.rs
│   ├── messages.rs
│   ├── peer.rs
│   ├── pieces.rs
│   ├── strategy.rs
│   └── client.rs
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

The `nw/` directory is the source of truth for Rust code. All `.rs` files are generated via `notangle` and should not be edited directly. Generated files include a header:
```rust
// GENERATED CODE - DO NOT EDIT
// Source: nw/01-bencode.nw
// Regenerate with: just tangle
```

`Cargo.toml` and `Cargo.lock` are **not generated**—they live in the repository root and are maintained manually. This reduces friction when adding dependencies.

### External Dependencies (Cargo crates, minimal)

- `tokio` — async runtime
- `reqwest` — async HTTP for tracker announces (consider `ureq` for simpler educational alternative)
- `sha1` — SHA1 hashing for info hash and piece verification
- `clap` — CLI argument parsing
- `bytes` — efficient byte buffer manipulation
- `thiserror` — ergonomic error types
- `tracing` — structured logging and observability
- `tracing-subscriber` — log output formatting

No BitTorrent protocol crates. No abstraction layers. Direct protocol implementation.

### Document Structure

The `.nw` files follow this narrative arc:

```
1. Introduction
   - What is BitTorrent?
   - Protocol overview diagram (tracker, peers, pieces)
   - Document roadmap

2. Bencode
   - Grammar definition
   - Decoder implementation
   - Encoder implementation
   - Round-trip property

3. Metainfo Files (.torrent)
   - Structure and fields
   - Info hash computation (SHA1 of bencoded info dict)
   - Single-file vs multi-file format
   - Parsing implementation

4. Tracker Protocol
   - Announce URL construction
   - HTTP tracker request parameters:
     - info_hash, peer_id, port, uploaded, downloaded, left
     - event: started | stopped | completed
   - HTTP tracker response parsing
   - Compact peer list format (BEP-23): 6 bytes per peer
   - Announce intervals and re-announces

5. Peer Wire Protocol
   - Handshake (distinct from messages):
     - pstrlen (1 byte): 19
     - pstr (19 bytes): "BitTorrent protocol"
     - reserved (8 bytes): zeros
     - info_hash (20 bytes)
     - peer_id (20 bytes)
   - Message framing: 4-byte big-endian length prefix
   - Message types (length-prefixed, unlike handshake):
     - keep-alive (length=0)
     - choke (id=0) / unchoke (id=1)
     - interested (id=2) / not interested (id=3)
     - have (id=4) / bitfield (id=5)
     - request (id=6) / piece (id=7) / cancel (id=8)

6. Piece Management
   - Piece and block geometry:
     - piece_length from metainfo (e.g., 262144 = 256KB)
     - BLOCK_SIZE = 16384 bytes (2^14)
     - blocks_per_piece = ceil(piece_length / BLOCK_SIZE)
     - last piece may be smaller
   - Byte stream to file mapping for multi-file torrents
   - SHA1 verification after all blocks received
   - Disk I/O: seek + write, sparse files
   - Progress tracking

7. Download Strategy
   - Peer selection
   - Piece selection (rarest first)
   - Request pipelining (10-16 concurrent requests per peer)
   - Endgame mode (optional—request same blocks from multiple peers)

8. Orchestration
   - Main event loop
   - Peer connection state machine (4-bit state)
   - Choking algorithm (simplified: unchoke interested peers with good rates)
   - Completion detection
   - Graceful shutdown (send event=stopped to tracker)

9. CLI Interface
   - Argument parsing
   - Progress display
   - Graceful shutdown (Ctrl+C handling)

10. Appendices
    - Protocol references (BEP links)
    - Test vectors (example bencoded values, expected hashes)
    - Future work (DHT, PEX, uTP, etc.)
```

### File Outputs (from tangle)

```
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

# Quick check (tangle + cargo check, no full build)
just check

# Build
just build

# Run tests
just test

# Run linter
just lint

# Check formatting
just fmt

# Fix formatting
just fmt-fix

# Weave documentation (LuaLaTeX)
just weave

# Fast weave (draft mode, no syntax highlighting)
just weave-fast

# Download a torrent
just run path/to/file.torrent

# Full rebuild (tangle + build + weave)
just all

# Run all checks (tangle + build + test + lint)
just ci

# End-to-end test (downloads a real torrent)
just e2e

# Watch mode (re-tangle and build on .nw changes)
just watch

# Clean generated artifacts
just clean

# Open generated PDF
just view
```

### Testing Strategy

- **Unit tests**: Bencode round-trips, metainfo parsing against known `.torrent` files, message serialization (in-module `#[cfg(test)]`, tangled from `.nw` files)
- **Integration tests**: Download a small, well-seeded public domain torrent (e.g., a Linux ISO)
- **Property tests**: Bencode encode/decode inverse property with `proptest`
- **Fuzz testing**: Consider `cargo-fuzz` targets for bencode parser and message parser

Integration tests live in a separate `tests/` directory (not literate—they're verification, not narrative).

**Verification Criteria per Milestone**:
- Phase 1 (Bencode): Property tests pass, round-trip for all value types
- Phase 2 (Tracker): Can parse real tracker responses, handles compact format
- Phase 3 (Messages): Can serialize/deserialize all message types
- Phase 4 (Pieces): Can verify piece hashes from real torrent
- Phase 5 (Strategy): Rarest-first selection works with simulated bitfields
- Phase 6 (Orchestration): Downloads complete torrent in controlled environment

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

### Contributor Guidance

**Literate Programming Workflow**

This project uses noweb for literate programming. Key points:

1. **Never edit files in `src/`** — they are generated and will be overwritten
2. **Edit `.nw` files in `nw/`** — this is the source of truth
3. **Run `just tangle`** before building to regenerate Rust code
4. **Run `just watch`** during development for automatic re-tangling

**IDE Setup**

rust-analyzer works on the tangled `src/` directory. Workflow:
1. Run `just tangle` once to generate initial source
2. Open project in IDE—rust-analyzer indexes `src/`
3. Edit `.nw` files, run `just tangle`, IDE picks up changes
4. Errors reference `src/*.rs` line numbers—map back to `.nw` chunks manually

**Debugging Tip**: Each generated file contains chunk markers as comments. Search for the chunk name to find the corresponding `.nw` source.

**Adding Dependencies**: Edit `Cargo.toml` directly (it is not generated). Then reference the crate in your `.nw` code chunks.

### References

- [BEP 3: The BitTorrent Protocol Specification](http://www.bittorrent.org/beps/bep_0003.html)
- [BEP 23: Tracker Returns Compact Peer Lists](http://www.bittorrent.org/beps/bep_0023.html)
- [Unofficial BitTorrent Specification (wiki.theory.org archive)](https://wiki.theory.org/BitTorrentSpecification)
- [Knuth, Literate Programming (1984)](http://www.literateprogramming.com/knuthweb.pdf)
- [noweb documentation](https://www.cs.tufts.edu/~nr/noweb/)

### License

MIT for code and documentation. This is meant to be studied and adapted.
