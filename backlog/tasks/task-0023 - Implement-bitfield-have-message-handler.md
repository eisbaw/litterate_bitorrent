---
id: task-0023
title: Implement bitfield/have message handler
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 19:19'
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
Handles incoming bitfield and have messages from peers. Updates the peer's known piece availability in the strategy module. This is foundational for piece selection - we cannot request pieces until we know what peers have.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 handle_bitfield() parses bitfield message and updates peer's piece availability
- [x] #2 handle_have() updates single piece availability for a peer
- [x] #3 Validates bitfield length matches expected piece count
- [x] #4 Updates strategy module with new availability data
- [x] #5 Sets am_interested=true if peer has pieces we need, sends interested message
- [x] #6 Unit test: bitfield parsing with various piece counts including edge cases
- [x] #7 Unit test: have message updates availability correctly
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented higher-level bitfield/have message handlers in nw/07-client.nw.

Functions added:
- handle_incoming_bitfield(conn, bits, availability, piece_manager) -> Result<bool, PeerError>
  Updates peer bitfield, global availability, returns true if should send Interested

- handle_incoming_have(conn, index, availability, piece_manager) -> bool
  Updates peer bitfield and availability, returns true if we need the piece

Also added PieceManager::is_verified(index) in nw/05-pieces.nw.

Design: Functions return boolean for interest decision, letting caller handle sending.
This keeps handlers pure and testable.

Tests added (8): availability updates, interest logic, invalid index handling, wrong length rejection.

All 680 tests pass.
<!-- SECTION:NOTES:END -->
