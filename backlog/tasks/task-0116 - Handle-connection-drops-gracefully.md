---
id: task-0116
title: Handle connection drops gracefully
status: To Do
assignee: []
created_date: '2026-01-16 21:39'
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
- [ ] #1 Read operations return Error::ConnectionClosed when stream closes (read returns 0 bytes)
- [ ] #2 Write operations return Error::IoError when connection reset or broken pipe
- [ ] #3 Partial handshake (e.g., only 30 of 68 bytes received) returns appropriate error
- [ ] #4 Partial message (length prefix received but body incomplete) returns appropriate error
- [ ] #5 All error types are logged at DEBUG level with peer address context
- [ ] #6 No panics on any connection failure path
<!-- AC:END -->
