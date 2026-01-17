---
id: task-0073
title: Implement message parsing (from_bytes)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 09:34'
labels:
  - phase-3
  - protocol
  - messages
  - parsing
dependencies:
  - task-0052
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse messages from bytes received from peers. First read 4-byte length prefix, then read that many bytes for the message body. Handle keep-alive (length=0) specially. Validate message ID and payload length for each type. Return clear errors for malformed messages.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Message::from_bytes(bytes: &[u8]) -> Result<Message, Error> parses a complete message (including length prefix)
- [x] #2 Returns KeepAlive when length=0
- [x] #3 Returns error for unknown message ID
- [x] #4 Validates Have payload is exactly 4 bytes
- [x] #5 Validates Request/Cancel payload is exactly 12 bytes
- [x] #6 Validates Piece payload is at least 8 bytes (index + begin)
- [x] #7 Returns InvalidMessage error with context for malformed input
- [x] #8 Unit tests cover valid messages and each validation failure case
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study existing messages.nw structure and wire format specifications
2. Add from_bytes method chunk to message impl section
3. Implement parsing logic: read length prefix, handle keep-alive, parse message ID and payload
4. Add validation for each message type (Have=4 bytes, Request/Cancel=12 bytes, Piece>=8 bytes)
5. Return appropriate PeerError variants for invalid input
6. Write literate prose explaining the parsing algorithm
7. Add comprehensive test chunk covering valid and invalid messages
8. Verify with nix-shell --run "just check && just test"
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented Message::from_bytes method for parsing BitTorrent peer wire protocol messages from bytes.

Key implementation details:
- Parses 4-byte big-endian length prefix first
- Returns KeepAlive for length=0 (no ID, no payload)
- Validates message ID is in range 0-8, returns UnexpectedMessageId for unknown IDs
- Validates payload lengths per message type:
  - Choke/Unchoke/Interested/NotInterested: no payload
  - Have: exactly 4 bytes
  - Request/Cancel: exactly 12 bytes
  - Piece: at least 8 bytes (index + begin)
  - Bitfield: any length
- Returns InvalidMessage with context for malformed input

Files modified:
- nw/04-messages.nw: Added from_bytes implementation, parsing prose, and comprehensive test suite

Tests added:
- Valid parsing for all 10 message types
- Round-trip tests (to_bytes -> from_bytes)
- Error cases: empty input, truncated, extra bytes, unknown ID
- Payload validation failures for each type
- Edge cases: max values, minimum valid sizes
<!-- SECTION:NOTES:END -->
