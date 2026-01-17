---
id: task-0077
title: Implement compact peer list parsing (BEP-23)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-17 08:46'
labels:
  - tracker
  - parsing
  - bep23
dependencies:
  - task-0069
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse the compact peer format where each peer is exactly 6 bytes: 4 bytes for IPv4 address (big-endian) and 2 bytes for port (big-endian). This is the preferred format (BEP-23) and most trackers support it when compact=1 is requested. The parser must handle arbitrary numbers of peers and validate the byte length is divisible by 6.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Function takes &[u8], returns Result<Vec<SocketAddr>, TrackerError>
- [x] #2 Each 6-byte chunk parsed as 4-byte IPv4 + 2-byte port (big-endian)
- [x] #3 Returns error if byte length not divisible by 6
- [x] #4 Empty input returns empty Vec (valid: no peers available)
- [x] #5 Unit test: parse 12 bytes into 2 peers with correct IP:port
- [x] #6 Unit test: parse 0 bytes returns empty vec
- [x] #7 Unit test: parse 7 bytes returns error (not divisible by 6)
- [x] #8 Unit test: verify endianness with known test vector
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add compact peer parsing section to nw/03-tracker.nw after the response parsing section
2. Implement parse_compact_peers function that takes &[u8] and returns Result<Vec<SocketAddr>, TrackerError>
3. Write literate prose explaining BEP-23 compact format (6 bytes per peer: 4-byte IPv4 + 2-byte port, big-endian)
4. Add comprehensive unit tests for all acceptance criteria
5. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Summary
- Added `parse_compact_peers` function to parse BEP-23 compact peer format
- Function takes `&[u8]` and returns `Result<Vec<SocketAddr>, TrackerError>`
- Added comprehensive literate prose explaining the BEP-23 format (6 bytes per peer: 4-byte IPv4 + 2-byte port, big-endian)
- Added `COMPACT_PEER_SIZE` constant (6) for clarity

## Implementation Details
- Uses `Ipv4Addr::new()` for natural byte order parsing of IPv4 addresses
- Uses `u16::from_be_bytes()` for explicit big-endian port parsing
- Returns error if byte length not divisible by 6
- Empty input returns empty `Vec` (valid: no peers available)
- Pre-allocates vector capacity for efficiency

## Tests Added
- `parse_compact_peers_two_peers`: Parse 12 bytes into 2 peers with correct IP:port
- `parse_compact_peers_empty_returns_empty_vec`: Parse 0 bytes returns empty vec
- `parse_compact_peers_not_divisible_by_6_returns_error`: Parse 7 bytes returns error
- `parse_compact_peers_endianness_verification`: Verify endianness with known test vector
- Additional edge case tests for min/max values, single peer, many peers, and various invalid lengths

## Files Modified
- nw/03-tracker.nw: Added compact peer parsing section with implementation and tests
<!-- SECTION:NOTES:END -->
