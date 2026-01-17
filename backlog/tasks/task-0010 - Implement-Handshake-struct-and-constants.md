---
id: task-0010
title: Implement Handshake struct and constants
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 09:05'
labels:
  - phase-3
  - protocol
  - handshake
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The handshake is a fixed 68-byte structure sent at connection start, distinct from the length-prefixed message framing used afterward. It consists of: pstrlen (1 byte, value 19), pstr (19 bytes, 'BitTorrent protocol'), reserved (8 bytes, zeros), info_hash (20 bytes), peer_id (20 bytes). This is the foundation for peer identification and torrent validation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Handshake struct with fields: info_hash: [u8; 20], peer_id: [u8; 20]
- [x] #2 Constant PROTOCOL_STRING = b"BitTorrent protocol" (19 bytes)
- [x] #3 Constant HANDSHAKE_LENGTH = 68
- [x] #4 Reserved bytes are 8 zeros (no extension bits for now)
- [x] #5 Handshake::new(info_hash, peer_id) constructor validates inputs
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Understand existing messages module structure and noweb patterns
2. Design Handshake struct with info_hash and peer_id fields
3. Add PROTOCOL_STRING and HANDSHAKE_LENGTH constants
4. Write literate prose explaining the 68-byte handshake format
5. Implement Handshake::new constructor with validation
6. Add comprehensive tests
7. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented Handshake struct and protocol constants per BEP 3 specification.

## Changes

- Added PROTOCOL_STRING constant (b"BitTorrent protocol", 19 bytes)
- Added HANDSHAKE_LENGTH constant (68 bytes = 1 + 19 + 8 + 20 + 20)
- Added Handshake struct with info_hash and peer_id fields ([u8; 20] each)
- Implemented Handshake::new() constructor with compile-time size validation via Rust types
- Added reserved() method returning 8 zero bytes (no extensions yet)
- Added comprehensive test suite (8 tests)

## Design Notes

- The Handshake struct stores only variable fields (info_hash, peer_id); fixed fields (protocol string, reserved) are generated on serialization
- Used Rust type system for input validation (arrays enforce exact 20-byte size at compile time)
- Derived Clone, PartialEq, Eq, Debug for ergonomic usage
- All public methods marked #[must_use] to catch likely bugs

## Wire Format Documentation

Included detailed literate prose explaining the 68-byte handshake format with ASCII diagram.
<!-- SECTION:NOTES:END -->
