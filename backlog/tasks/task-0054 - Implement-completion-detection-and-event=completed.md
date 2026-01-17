---
id: task-0054
title: Implement completion detection and event=completed
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 17:49'
labels:
  - phase-6
  - completion
  - tracker
dependencies:
  - task-0031
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Detects when all pieces have been downloaded and verified. Sends event=completed to tracker to transition from leecher to seeder status (though we do not actually seed per non-goals).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Check after each piece verification if all pieces are complete
- [x] #2 Send tracker announce with event=completed when download finishes
- [x] #3 Log completion at INFO level with total time and average speed
- [x] #4 Set internal state to prevent further requesting
- [x] #5 Handle tracker failure on completed announce (log warning, not fatal)
- [x] #6 Unit test: completion detected when last piece verifies
- [x] #7 Integration test: completed event sent to tracker
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add CompletionHandler struct to track completion state and timing in client.rs
2. Add send_completed_announce() async function similar to send_stopped_announce()
3. Add on_piece_verified() method to Client that checks is_complete() after each verification
4. Log completion with INFO level including total time and average speed
5. Add internal completed flag to prevent further requesting
6. Handle tracker failure on completed announce (log warning, continue)
7. Add unit tests for completion detection
8. Add integration test for completed event sent to tracker
9. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Summary

Implemented download completion detection in the client module. Key changes:

### New Client Fields
- `start_time: Instant` - tracks when download began for speed calculation
- `download_complete: bool` - prevents further piece requests after completion
- `completed_announced: bool` - ensures event=completed sent exactly once

### New Public API
- `check_completion() -> CompletionResult` - call after each piece verification
- `mark_completed_announced()` - marks announce as sent
- `send_completed_announce()` - async function to notify tracker
- `CompletionResult` enum: AlreadyComplete, StillDownloading, JustCompleted

### Logging
INFO level completion log includes total bytes, elapsed time, average speed, and piece count.

### Error Handling
Tracker failure on completed announce logs warning but does not fail the download.

### Tests Added
- 16 unit tests covering completion detection, state transitions, and format helpers
- 1 async integration test for tracker communication with unreachable tracker
<!-- SECTION:NOTES:END -->
