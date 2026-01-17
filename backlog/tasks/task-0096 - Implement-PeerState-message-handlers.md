---
id: task-0096
title: Implement PeerState message handlers
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 18:05'
labels:
  - phase-3
  - protocol
  - peer
  - state
dependencies:
  - task-0088
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
PeerState must update correctly when receiving protocol messages. Each message type changes specific state: choke/unchoke updates peer_choking, interested/not_interested updates peer_interested, have/bitfield updates the bitfield, etc. This is the state machine core.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PeerState::handle_choke() sets peer_choking=true and clears pending_requests
- [x] #2 PeerState::handle_unchoke() sets peer_choking=false
- [x] #3 PeerState::handle_interested() sets peer_interested=true
- [x] #4 PeerState::handle_not_interested() sets peer_interested=false
- [x] #5 PeerState::handle_have(index: u32) sets bit in bitfield, validates index
- [x] #6 PeerState::handle_bitfield(bits: &[u8]) -> Result<(), Error> sets bitfield, validates length matches num_pieces
- [x] #7 Unit tests verify state transitions for each handler
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add message handler methods to PeerState impl block (after set_piece method)
2. Implement handle_choke() - sets peer_choking=true, clears pending_requests
3. Implement handle_unchoke() - sets peer_choking=false
4. Implement handle_interested() - sets peer_interested=true
5. Implement handle_not_interested() - sets peer_interested=false
6. Implement handle_have(index: u32) -> bool - sets bit in bitfield, validates bounds
7. Implement handle_bitfield(bits: &[u8]) -> Result<(), PeerError> - replaces bitfield, validates length
8. Add unit tests for each handler
9. Run tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented message handler methods for PeerState in nw/07-client.nw.

Methods added:
- handle_choke() - sets peer_choking=true and clears pending_requests
- handle_unchoke() - sets peer_choking=false
- handle_interested() / handle_not_interested() - updates peer_interested flag
- handle_have(index) - delegates to set_piece() with bounds checking
- handle_bitfield(bits) - validates length, returns PeerError::BitfieldWrongLength on mismatch

Tests added: 10 tests covering all state transitions and edge cases.

All 639 tests pass.
<!-- SECTION:NOTES:END -->
