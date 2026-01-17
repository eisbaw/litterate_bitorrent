---
id: task-0056
title: Implement block selection within a piece
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 16:53'
labels:
  - phase-5
  - strategy
  - selection
dependencies:
  - task-0004
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Once a piece is selected, determine which blocks to request. Blocks within a piece are requested sequentially (offset 0, then BLOCK_SIZE, then 2*BLOCK_SIZE, etc.). Must track which blocks are already requested/received to avoid duplicates. Last block of a piece may be smaller than BLOCK_SIZE.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 select_next_blocks() returns list of (offset, length) pairs for unrequested blocks in a piece
- [x] #2 Respects BLOCK_SIZE constant (16384 bytes)
- [x] #3 Calculates correct length for final block when piece_length is not multiple of BLOCK_SIZE
- [x] #4 Skips blocks already marked as received in piece state
- [x] #5 Skips blocks already in-flight (pending request)
- [x] #6 Unit test: 64KB piece produces 4 blocks of 16KB each
- [x] #7 Unit test: 50KB piece produces 3 blocks of 16KB and 1 block of 2KB
- [x] #8 Unit test: skips already-received blocks correctly
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read existing code to understand PieceState, blocks_per_piece, and block_size functions
2. Design select_next_blocks() function signature and semantics
3. Write literate prose explaining the block selection algorithm
4. Implement the function using existing geometry utilities
5. Add unit tests for: 64KB piece (4 blocks), 50KB piece (3 full + 1 partial), skipping received blocks, skipping in-flight blocks
6. Run just check && just test to verify
7. Update acceptance criteria in task
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented select_next_blocks() function in the pieces module (nw/05-pieces.nw).

Key changes:
- Added BlockDescriptor type alias for (offset, length) pairs
- Added select_next_blocks() function that:
  - Takes PieceManager, piece_index, in_flight_offsets HashSet, and max_blocks limit
  - Returns Vec<(offset, length)> for blocks needing requests
  - Skips blocks already received (tracked in PieceState::InProgress bit vector)
  - Skips blocks with offsets in the in_flight_offsets set
  - Calculates correct length for final block when piece is not multiple of BLOCK_SIZE
  - Returns empty vec for Missing, Verified, or Failed pieces

Tests added (13 new tests):
- 64KB piece produces 4 blocks of 16KB each
- 50KB piece produces 3 blocks of 16KB and 1 block of 2KB
- Skips already-received blocks correctly
- Skips in-flight blocks correctly
- Respects max_blocks limit
- Returns empty for Missing, Verified, invalid index
- Handles single-block pieces and partial last pieces

All 131 pieces tests pass. Pre-existing failure in cli::logging_tests is unrelated.
<!-- SECTION:NOTES:END -->
