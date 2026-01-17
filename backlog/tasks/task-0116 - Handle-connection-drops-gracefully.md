---
id: task-0116
title: Handle connection drops gracefully
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:39'
updated_date: '2026-01-17 22:04'
labels:
  - phase-3
  - network
  - error-handling
dependencies:
  - task-0106
  - task-0109
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
TCP connections can close unexpectedly at any point: during handshake, while reading messages, or during writes. The peer module must handle all these cases cleanly, logging appropriately and returning errors that allow the orchestration layer to try other peers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Read operations return Error::ConnectionClosed when stream closes (read returns 0 bytes)
- [x] #2 Write operations return Error::IoError when connection reset or broken pipe
- [x] #3 Partial handshake (e.g., only 30 of 68 bytes received) returns appropriate error
- [x] #4 Partial message (length prefix received but body incomplete) returns appropriate error
- [x] #5 All error types are logged at DEBUG level with peer address context
- [x] #6 No panics on any connection failure path
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review current implementations of read_message, write_message, perform_handshake
2. Analyze acceptance criteria against current code
3. Add DEBUG-level logging with peer address context to read_message
4. Add DEBUG-level logging with peer address context to write_message
5. Add DEBUG-level logging with peer address context to perform_handshake for partial reads
6. Run tests and lint to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented graceful connection drop handling in nw/07-client.nw:

1. Fixed perform_handshake() to explicitly handle UnexpectedEof (partial handshake scenario):
   - Added explicit match on read_exact and write_all errors
   - UnexpectedEof now returns PeerError::ConnectionClosed instead of IoError
   - Added comment explaining partial handshake handling (e.g., 30 of 68 bytes)
   - Updated function docstring to document ConnectionClosed error variant

2. Changed logging level for connection errors from warn\! to debug\!:
   - error receiving message (line ~7661)
   - failed to send interested (lines ~7723, ~7742)
   - failed to send cancel (line ~7948)
   - failed to send request (lines ~7991, ~8097)
   - failed to send keep-alive (line ~8035)

3. Kept warn\! level for actual protocol errors:
   - Invalid bitfield (protocol violation, indicates buggy peer)
   - Failed to mark piece verified/failed (internal state errors)
   - Tracker errors

Rationale: Connection drops are normal in P2P systems - peers come and go constantly.
DEBUG level is appropriate for routine events, while WARN is reserved for actual problems.

All existing tests pass (758 unit tests, 90 doc tests). Lint passes with no warnings.
<!-- SECTION:NOTES:END -->
