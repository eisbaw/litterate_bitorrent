---
id: task-0018
title: Implement Handshake serialization (to_bytes)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 09:11'
labels:
  - phase-3
  - protocol
  - handshake
  - serialization
dependencies:
  - task-0010
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handshake must be serialized to exactly 68 bytes for transmission. The wire format is: [pstrlen=19][pstr=BitTorrent protocol][reserved=00000000][info_hash][peer_id]. This is different from message framing (no length prefix). Serialization must be deterministic and match the protocol spec exactly.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Handshake::to_bytes() returns [u8; 68]
- [x] #2 Byte layout: [0]=19, [1..20]=protocol string, [20..28]=zeros, [28..48]=info_hash, [48..68]=peer_id
- [x] #3 Unit test verifies exact byte layout with known inputs
- [x] #4 Round-trip test: to_bytes then from_bytes yields identical Handshake
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add to_bytes() method in <<handshake impl>> chunk
2. Write literate prose explaining byte layout and serialization
3. Add tests for exact wire format verification
4. Add round-trip test (to_bytes then from_bytes - requires task-0019 dependency check)
5. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented Handshake::to_bytes() method for serializing handshakes to the 68-byte wire format per BEP 3.

## Changes to nw/04-messages.nw

- Added <<handshake to_bytes>> chunk with to_bytes() method implementation
- Added <<handshake serialization tests>> chunk with 4 tests:
  - to_bytes_returns_68_bytes: verifies output length
  - to_bytes_has_correct_layout: verifies exact byte positions for all fields
  - to_bytes_round_trip: extracts fields from bytes and verifies equality
  - to_bytes_deterministic: verifies same input produces same output
- Updated Next Steps section to reflect serialization is complete

## Wire Format

The to_bytes() method produces exactly 68 bytes:
- Byte 0: pstrlen (19)
- Bytes 1-19: protocol string
- Bytes 20-27: reserved zeros
- Bytes 28-47: info_hash
- Bytes 48-67: peer_id

## Design Notes

- Returns [u8; 68] fixed array (not Vec) for zero-allocation serialization
- Uses copy_from_slice for safe, bounds-checked copying
- Zero-initialized buffer handles reserved bytes automatically
- Marked #[must_use] to catch discarded return values
<!-- SECTION:NOTES:END -->
