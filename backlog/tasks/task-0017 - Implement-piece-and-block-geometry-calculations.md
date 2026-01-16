---
id: task-0017
title: Implement piece and block geometry calculations
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
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
- [ ] #1 BLOCK_SIZE constant defined as 16384 bytes (2^14)
- [ ] #2 blocks_per_piece(piece_length) returns ceiling division of piece_length by BLOCK_SIZE
- [ ] #3 piece_size(piece_index, piece_length, total_length, num_pieces) returns actual size of piece (handles last piece being smaller)
- [ ] #4 block_size(piece_index, block_index, piece_length, total_length, num_pieces) returns actual block size (handles last block being smaller)
- [ ] #5 Unit tests cover: normal piece, last piece partial, single-piece torrent, last block of last piece
<!-- AC:END -->
