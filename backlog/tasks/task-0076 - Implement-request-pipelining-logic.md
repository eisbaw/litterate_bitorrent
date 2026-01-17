---
id: task-0076
title: Implement request pipelining logic
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-17 20:02'
labels:
  - phase-5
  - strategy
  - pipelining
dependencies:
  - task-0056
  - task-0067
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Request pipelining sends multiple block requests before receiving responses, maximizing throughput by keeping the network pipe full. The strategy must fill each peer's request queue up to capacity with blocks from selected pieces, respecting piece selection order.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 fill_request_queue() generates requests to fill peer queue to capacity
- [x] #2 Prioritizes completing in-progress pieces before starting new ones
- [x] #3 Uses rarest-first for selecting new pieces to start
- [x] #4 Returns list of BlockRequest to send to peer
- [x] #5 Respects peer's bitfield (only request pieces they have)
- [x] #6 Unit test: fills queue to exactly max_size when sufficient blocks available
- [x] #7 Unit test: prefers blocks from in-progress pieces over starting new pieces
- [x] #8 Unit test: returns empty when peer has no pieces we need
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read PieceManager API (get_state, get_piece_size, start_piece, num_pieces)
2. Read select_rarest function signature and usage
3. Write fill_request_queue function in new chunk
4. Write select_next_block helper function
5. Write find_unrequested_block helper function
6. Add literate prose explaining pipelining strategy
7. Add unit tests for all acceptance criteria
8. Add chunk reference in client.rs assembly
9. Run tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented request pipelining logic in nw/07-client.nw.

Functions:
- fill_request_queue() - generates requests to fill peer queue to capacity
- select_next_block() - selects next block using two-phase strategy
- find_unrequested_block() - finds unreceived, unrequested blocks in piece

Two-phase strategy:
1. Complete in-progress pieces the peer has (reduces memory pressure)
2. Start new pieces using rarest-first selection

Key design:
- Returns empty when peer is choking us
- Respects peer bitfield (only request pieces they have)
- Deduplicates (skips already-queued blocks)
- Backpressure from PeerRequestQueue capacity

Tests added (7): capacity fill, in-progress priority, choke handling, rarest-first.

All 711 tests pass.
<!-- SECTION:NOTES:END -->
