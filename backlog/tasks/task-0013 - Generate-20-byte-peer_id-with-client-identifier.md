---
id: task-0013
title: Generate 20-byte peer_id with client identifier
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 08:21'
labels:
  - tracker
  - identity
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each BitTorrent client instance needs a unique 20-byte peer_id sent to trackers and peers. The peer_id identifies this specific client session. Convention is to start with a client identifier (e.g., -LT0001- for Literate Torrent v0.0.01) followed by random bytes to ensure uniqueness across sessions. The format helps debugging and client identification in swarms.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Function generates 20-byte peer_id as [u8; 20]
- [x] #2 First 8 bytes follow Azureus-style convention: -LT0001- (or similar client prefix)
- [x] #3 Remaining 12 bytes are random (use rand crate or getrandom)
- [x] #4 peer_id is generated once per client session and reused
- [x] #5 Unit test: generated peer_id is exactly 20 bytes
- [x] #6 Unit test: peer_id prefix matches expected client identifier
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add rand crate dependency to Cargo.toml for random byte generation
2. Write literate prose explaining BEP 20 peer_id format (Azureus-style)
3. Implement generate_peer_id function returning [u8; 20]
4. Add tests for length (20 bytes) and prefix (-LT0001-)
5. Tangle and verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added peer_id generation to nw/03-tracker.nw with:

- PEER_ID_PREFIX constant ("-LT0001-") following Azureus-style BEP 20 format
- generate_peer_id() function returning [u8; 20] with prefix + 12 random bytes
- Literate prose explaining BEP 20 peer_id conventions, format structure, and rationale
- Four unit tests verifying length, prefix, randomness, and constant correctness

Dependencies: Added rand v0.8 crate for cryptographically secure random byte generation.

Verification: All 169 tests pass, clippy clean, cargo check succeeds.
<!-- SECTION:NOTES:END -->
