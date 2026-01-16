---
id: task-0067
title: Implement bounded request queue per peer
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
labels:
  - phase-5
  - strategy
  - pipelining
dependencies:
  - task-0004
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each peer connection maintains a bounded queue of in-flight requests. The queue size (typically 10-16) limits concurrent requests to prevent overwhelming peers and provides backpressure. Queue tracks BlockRequest structs with timestamps for timeout detection.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PeerRequestQueue struct with configurable max_size (default 16)
- [ ] #2 push() adds request if queue not full, returns false if full
- [ ] #3 remove() removes request matching (piece_index, offset)
- [ ] #4 is_full() returns true when queue at capacity
- [ ] #5 len() returns current queue size
- [ ] #6 iter() allows iteration over pending requests (for timeout checking)
- [ ] #7 Unit test: queue rejects push when at max_size
- [ ] #8 Unit test: remove correctly identifies and removes matching request
- [ ] #9 Unit test: queue maintains FIFO order
<!-- AC:END -->
