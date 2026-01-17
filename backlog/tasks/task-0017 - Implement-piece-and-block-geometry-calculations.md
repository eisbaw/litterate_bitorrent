---
id: task-0017
title: Implement piece and block geometry calculations
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 09:52'
labels:
  - phase4
  - foundation
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the core geometry calculations that underpin all piece management. This includes computing blocks per piece, handling the last piece (which may be smaller), and handling the last block of each piece (also may be smaller). These are pure functions with no state, making them easy to test in isolation. The calculations must handle edge cases: single-piece torrents, torrents where piece_length equals total_length, and the common case where the last piece is partial.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 BLOCK_SIZE constant defined as 16384 bytes (2^14)
- [x] #2 blocks_per_piece(piece_length) returns ceiling division of piece_length by BLOCK_SIZE
- [x] #3 piece_size(piece_index, piece_length, total_length, num_pieces) returns actual size of piece (handles last piece being smaller)
- [x] #4 block_size(piece_index, block_index, piece_length, total_length, num_pieces) returns actual block size (handles last block being smaller)
- [x] #5 Unit tests cover: normal piece, last piece partial, single-piece torrent, last block of last piece
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define BLOCK_SIZE constant (16384 bytes = 2^14)
2. Implement blocks_per_piece(piece_length) using ceiling division
3. Implement piece_size(piece_index, piece_length, total_length, num_pieces) for last-piece handling
4. Implement block_size(piece_index, block_index, piece_length, total_length, num_pieces) for last-block handling
5. Add block_offset(block_index) helper function
6. Write comprehensive unit tests for edge cases
7. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented piece and block geometry calculations in the pieces module:

- Added BLOCK_SIZE constant (16384 bytes = 2^14) as the de facto BitTorrent block size
- Added blocks_per_piece(piece_length) using ceiling division for partial last blocks
- Added piece_size(piece_index, piece_length, total_length, num_pieces) to handle last piece being smaller
- Added block_size(piece_index, block_index, piece_length, total_length, num_pieces) to handle last block of each piece being smaller
- Added block_offset(block_index) helper for calculating byte offset within a piece

All functions are pure, #[inline], and panic on invalid indices (with clear error messages). Edge cases covered:
- Single-piece torrents
- Single-block pieces
- Last piece partial
- Last block partial
- One-byte torrents

22 new unit tests added covering all edge cases. All 397 tests pass.
<!-- SECTION:NOTES:END -->
