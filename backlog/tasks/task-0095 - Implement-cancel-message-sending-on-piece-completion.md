---
id: task-0095
title: Implement cancel message sending on piece completion
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 23:23'
labels:
  - phase-5
  - strategy
  - cancel
dependencies:
  - task-0067
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When a piece is completed by receiving the final block from one peer, any pending requests for blocks of that piece to other peers must be cancelled. This avoids wasting bandwidth and allows those peers to serve other requests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 cancel_piece_requests() takes piece_index and list of all peer queues
- [x] #2 Returns list of (peer_id, CancelMessage) pairs for all pending requests
- [x] #3 Removes cancelled requests from each peer's queue
- [x] #4 Does not cancel from the peer that completed the piece (they already delivered)
- [x] #5 Unit test: cancels requests from multiple peers correctly
- [x] #6 Unit test: excludes the completing peer from cancellation
- [x] #7 Unit test: handles case where no other peers had pending requests
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a remove_for_piece() method to PeerRequestQueue that removes and returns all requests for a given piece
2. Add cancel_piece_requests() function that iterates over all peer queues except the completing peer
3. For each queue, call remove_for_piece() and convert results to (SocketAddr, Message::Cancel) pairs
4. Add unit tests for the new functionality
5. Run lint and tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented cancel_piece_requests() function for cancelling pending block requests when a piece is completed.

Changes:
- Added remove_for_piece() method to PeerRequestQueue that removes and returns all requests for a given piece index
- Added cancel_piece_requests() function that iterates over all peer queues, skipping the completing peer, and generates Cancel messages for all pending requests for the completed piece
- Added comprehensive unit tests covering all acceptance criteria

The function:
1. Takes piece_index, completing_peer address, and mutable reference to request queues map
2. Returns Vec<(SocketAddr, Message::Cancel)> for the caller to send
3. Removes cancelled requests from each peer queue
4. Excludes the completing peer from cancellation

Files modified: nw/07-client.nw
<!-- SECTION:NOTES:END -->
