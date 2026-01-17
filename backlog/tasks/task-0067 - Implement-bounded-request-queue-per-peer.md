---
id: task-0067
title: Implement bounded request queue per peer
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 19:49'
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
- [x] #1 PeerRequestQueue struct with configurable max_size (default 16)
- [x] #2 push() adds request if queue not full, returns false if full
- [x] #3 remove() removes request matching (piece_index, offset)
- [x] #4 is_full() returns true when queue at capacity
- [x] #5 len() returns current queue size
- [x] #6 iter() allows iteration over pending requests (for timeout checking)
- [x] #7 Unit test: queue rejects push when at max_size
- [x] #8 Unit test: remove correctly identifies and removes matching request
- [x] #9 Unit test: queue maintains FIFO order
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add PeerRequestQueue struct and constants after peer id generation section (around line 85)
2. Add new chunk <<peer request queue>> with TimestampedRequest and PeerRequestQueue structs
3. Add unit tests in <<peer request queue tests>> chunk
4. Include both chunks in <<client.rs>> aggregation
5. Run just check && cargo test peer_request_queue to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented PeerRequestQueue in nw/07-client.nw.

Structs:
- TimestampedRequest: wraps BlockRequest with sent_at: Instant
- PeerRequestQueue: bounded FIFO queue with max_size

Methods:
- new(max_size), with_default_size()
- push(request) -> bool (false when full = backpressure)
- remove(piece_index, offset) -> Option<TimestampedRequest>
- is_full(), len(), is_empty()
- iter() for timeout checking
- timed_out(Duration) -> Vec of expired requests
- drain() -> Iterator of all requests

Constant: DEFAULT_MAX_REQUESTS_PER_PEER = 16

Design: Timestamps enable timeout detection, backpressure prevents peer overwhelm.

Tests added (10): capacity enforcement, FIFO order, matching removal, timeout detection.

All 703 tests pass (1 pre-existing flaky test).
<!-- SECTION:NOTES:END -->
