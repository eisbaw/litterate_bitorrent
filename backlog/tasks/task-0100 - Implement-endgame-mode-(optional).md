---
id: task-0100
title: Implement endgame mode (optional)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:34'
updated_date: '2026-01-18 03:01'
labels:
  - phase-5
  - strategy
  - endgame
  - optional
dependencies:
  - task-0095
  - task-0089
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Endgame mode activates when few pieces remain. In this mode, blocks are requested from multiple peers simultaneously to avoid stalling on slow peers. When a block arrives, cancel messages are sent to all other peers. This is optional per PRD but improves end-of-download performance.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 is_endgame_mode() returns true when remaining pieces below threshold (e.g., 5 or fewer pieces)
- [x] #2 In endgame mode, same block can be requested from multiple peers
- [x] #3 Track duplicate requests across peers
- [x] #4 On block receipt, cancel duplicates from other peers
- [x] #5 Threshold is configurable
- [x] #6 Unit test: endgame mode activates at correct piece count
- [x] #7 Unit test: duplicate requests are tracked correctly
- [x] #8 Unit test: cancellations sent on receipt during endgame
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add endgame_threshold to SchedulerConfig with default value of 5
2. Create EndgameTracker struct to track duplicate requests (HashMap<BlockKey, Vec<PeerId>>)
3. Add is_endgame_mode() method to check if remaining pieces <= threshold
4. Add register_endgame_request() method to allow duplicate requests in endgame mode
5. Add get_endgame_cancellations() method to return peers to cancel when block received
6. Write prose explaining endgame strategy and implementation
7. Add unit tests for all acceptance criteria
8. Run tangle, lint, and test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented endgame mode for the RequestScheduler.

## Changes

- Added `endgame_threshold` to SchedulerConfig (default 5 pieces)
- Added `endgame_requests` HashMap to track duplicate block requests across peers
- Added `is_endgame_mode(&PieceManager)` - returns true when remaining pieces <= threshold
- Added `register_endgame_request()` - allows same block to be requested from multiple peers
- Added `get_endgame_peers()` - returns peers with pending requests for a block
- Added `complete_endgame_request()` - completes a block and returns peers needing cancel messages

## Testing

- 9 unit tests covering all acceptance criteria
- Doc tests for the new public APIs
- All 895 existing tests pass

## Usage

```rust
if scheduler.is_endgame_mode(&manager) {
    scheduler.register_endgame_request(request);
} else {
    scheduler.register_request(request);
}

// On block receipt:
let (completed, cancel_peers) = scheduler.complete_endgame_request(piece, offset, &from_peer);
for peer in cancel_peers {
    // Send Cancel message to peer
}
```
<!-- SECTION:NOTES:END -->
