---
id: task-0094
title: Implement BlockRequest struct
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
labels:
  - phase-3
  - protocol
  - peer
  - requests
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Track individual block requests for timeout and cancellation. Each request records the piece index, byte offset within the piece, length, timestamp, and optionally which peer it was sent to. This enables request pipelining and timeout handling.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 BlockRequest struct with: piece_index: u32, begin: u32, length: u32, requested_at: Instant
- [ ] #2 BlockRequest::new(piece_index, begin, length) sets requested_at to Instant::now()
- [ ] #3 BlockRequest::is_timed_out(timeout: Duration) -> bool
- [ ] #4 BlockRequest::matches(index: u32, begin: u32, length: u32) -> bool for matching responses
- [ ] #5 Implement PartialEq based on piece_index, begin, length (ignore timestamp)
<!-- AC:END -->
