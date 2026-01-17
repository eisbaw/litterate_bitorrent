---
id: task-0083
title: Implement request timeout detection
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-17 20:09'
labels:
  - phase-5
  - strategy
  - timeout
dependencies:
  - task-0067
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Block requests that exceed timeout (30s per PRD) must be detected so they can be cancelled and re-requested from another peer. Timeout detection scans pending requests and identifies those past the deadline. This is separate from the action taken (cancellation).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 check_timeouts() returns list of timed-out BlockRequests
- [x] #2 Timeout threshold is configurable (default 30 seconds)
- [x] #3 Uses requested_at timestamp from BlockRequest for comparison
- [x] #4 Timed-out requests remain in queue until explicitly removed (caller's responsibility)
- [x] #5 Unit test: request at T=0, check at T=31s returns it as timed out
- [x] #6 Unit test: request at T=0, check at T=29s does not return it
- [x] #7 Unit test: multiple requests, only those past timeout are returned
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add REQUEST_TIMEOUT_SECS constant (30 seconds) after DEFAULT_MAX_REQUESTS_PER_PEER
2. Add check_timeouts() convenience method to PeerRequestQueue
3. Add check_timeouts_with(Duration) method to PeerRequestQueue
4. Add documentation prose explaining timeout design
5. Add unit tests for the new methods
6. Run just check && cargo test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented request timeout detection in nw/07-client.nw.

Constant: REQUEST_TIMEOUT_SECS = 30 (per PRD)

Methods added to PeerRequestQueue:
- check_timeouts() - returns timed-out requests using default timeout
- check_timeouts_with(Duration) - custom timeout support

Design: Detection is separate from removal. Caller decides what to do with timed-out requests (cancel, re-request elsewhere).

Tests added (5): constant value, empty for fresh, zero duration, custom duration, requests stay in queue.

All 716 tests pass.
<!-- SECTION:NOTES:END -->
