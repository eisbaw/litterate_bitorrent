---
id: task-0027
title: Implement choke/unchoke message handler
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 19:28'
labels:
  - phase-6
  - message-handler
  - peer-protocol
dependencies:
  - task-0006
  - task-0015
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles incoming choke and unchoke messages from peers. Choke state controls whether we can request blocks from a peer. When unchoked, we should start requesting; when choked, we must stop requesting and mark pending requests for re-request from other peers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 handle_unchoke() sets peer_choking=false, triggers block requesting
- [x] #2 handle_choke() sets peer_choking=true, cancels pending requests to this peer
- [x] #3 Choked requests are returned to the piece manager for reassignment
- [x] #4 Logs state transitions at INFO level per PRD
- [x] #5 Unit test: unchoke enables requesting, choke disables it
- [x] #6 Unit test: pending requests are properly cancelled on choke
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read and understand existing PeerState.handle_choke/handle_unchoke methods
2. Add tracing::info import to <<incoming message handlers>> section
3. Add handle_incoming_unchoke() function with logging
4. Add handle_incoming_choke() function that returns cancelled requests
5. Add format_peer_id helper function
6. Add tests for all new functions
7. Run just check && cargo test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented higher-level choke/unchoke handlers in nw/07-client.nw.

Functions:
- handle_incoming_unchoke(conn) - updates peer state, logs at INFO level
- handle_incoming_choke(conn) -> Vec<BlockRequest> - returns cancelled requests for reassignment
- format_peer_id(peer_id) - helper for logging (first 8 bytes as hex)

Key design:
- Drain pending_requests BEFORE handle_choke clears them
- Return cancelled requests for reassignment to other peers
- Only log on actual state transitions
- INFO level logging with peer_id and addr

Tests added (7): state transitions, request cancellation, format_peer_id

All 687 tests pass.
<!-- SECTION:NOTES:END -->
