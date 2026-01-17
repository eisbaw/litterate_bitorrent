---
id: task-0116
title: Handle connection drops gracefully
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:39'
updated_date: '2026-01-17 22:09'
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
All connection drop paths return appropriate errors. Logging at DEBUG for routine drops, WARN for protocol violations.
<!-- SECTION:NOTES:END -->
