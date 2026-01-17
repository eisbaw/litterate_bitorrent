---
id: task-0036
title: Implement Handshake parsing (from_bytes) with validation
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 09:17'
labels:
  - phase-3
  - protocol
  - handshake
  - parsing
dependencies:
  - task-0010
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse handshake from exactly 68 bytes received from a peer. Must validate: pstrlen is 19, pstr matches 'BitTorrent protocol' exactly, info_hash matches expected (critical security check to ensure peer is on same torrent). Invalid handshakes must fail fast with clear error messages.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Handshake::from_bytes(bytes: &[u8]) -> Result<Handshake, Error>
- [x] #2 Returns error if input length \!= 68
- [x] #3 Returns error if pstrlen \!= 19
- [x] #4 Returns error if pstr \!= b"BitTorrent protocol"
- [x] #5 Handshake::validate_info_hash(expected: &[u8; 20]) -> Result<(), Error>
- [x] #6 Unit tests cover: valid handshake, wrong pstrlen, wrong pstr, wrong length
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add from_bytes method to Handshake impl block in 04-messages.nw
2. Validate input length is exactly 68 bytes
3. Validate pstrlen byte is 19
4. Validate pstr matches PROTOCOL_STRING exactly
5. Extract reserved, info_hash, and peer_id bytes
6. Add validate_info_hash method for comparing against expected hash
7. Write literate prose explaining the validation logic
8. Add comprehensive unit tests covering all validation scenarios
9. Run just check && just test to verify implementation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented Handshake::from_bytes() with comprehensive validation:

- Length validation: rejects input \!= 68 bytes
- pstrlen validation: first byte must be 19
- pstr validation: bytes 1-19 must match "BitTorrent protocol" exactly
- Reserved bytes (20-27) are accepted without validation for forward compatibility
- Extracts info_hash (bytes 28-47) and peer_id (bytes 48-67)

Also implemented Handshake::validate_info_hash() for security-critical hash comparison.

Added comprehensive tests covering:
- Valid handshake parsing and round-trip
- Rejection of too-short and too-long buffers
- Rejection of wrong pstrlen and wrong pstr values
- Acceptance of non-zero reserved bytes
- Info hash matching and mismatch detection

All 264 tests pass.
<!-- SECTION:NOTES:END -->
