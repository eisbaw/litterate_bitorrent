---
id: task-0068
title: Handle tracker unreachable during download
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-18 02:51'
labels:
  - phase-6
  - recovery
  - tracker
dependencies:
  - task-0050
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles the case where tracker becomes unreachable during an active download. The download should continue with existing peers. This is part of graceful degradation per PRD recovery strategy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Re-announce failure does not stop the download
- [x] #2 Retry tracker with exponential backoff (30s, 60s, 120s, cap at 15 min)
- [x] #3 Continue downloading with existing peer pool
- [x] #4 Log tracker failure at WARN level
- [x] #5 Resume normal re-announce interval after successful retry
- [x] #6 Unit test: tracker failure triggers backoff
- [x] #7 Integration test: download completes despite tracker outage
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update constants: Add TRACKER_BACKOFF_BASE_SECS = 30 and TRACKER_BACKOFF_MAX_SECS = 900
2. Update handle_re_announce() to use fixed 30s base instead of tracker_interval
3. Add cap at 900 seconds (15 minutes)
4. Update documentation/prose in the noweb file to reflect new intervals
5. Update unit tests to verify the new 30s, 60s, 120s... 900s sequence
6. Add integration test (may use network annotation or mock)
7. Run just tangle && just lint && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated tracker re-announce backoff to use fixed intervals per the acceptance criteria.

Changes made:
- Added TRACKER_BACKOFF_BASE_SECS (30s) and TRACKER_BACKOFF_MAX_SECS (900s) constants
- Modified handle_re_announce() to use 30s * 2^(failures-1) formula instead of tracker_interval * 2^failures
- Backoff sequence: 30s, 60s, 120s, 240s, 480s, 900s (capped)
- Updated prose documentation to reflect the new intervals
- Added comprehensive unit tests for backoff calculation

Key files modified:
- nw/07-client.nw: Updated constants, handle_re_announce(), tests, and documentation

All 886 unit tests + 93 doc tests pass. The implementation verifies that:
- First failure retries in 30s
- Subsequent failures double the interval
- Backoff caps at 15 minutes (900s)
- Success resets the failure count
<!-- SECTION:NOTES:END -->
