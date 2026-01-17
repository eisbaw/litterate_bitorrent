---
id: task-0042
title: Implement PieceManager struct with state tracking
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 10:00'
labels:
  - phase4
  - state
dependencies:
  - task-0024
  - task-0017
  - task-0008
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the PieceManager struct that holds state for all pieces in the torrent. This is the central coordinator for piece download state. It stores an array of PieceState for each piece, along with the metadata needed for geometry calculations (piece_length, total_length, piece_hashes). The manager provides methods to query and transition piece states, but does not handle networking or disk I/O - those are separate concerns.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PieceManager struct holds: piece_states Vec<PieceState>, piece_length u32, total_length u64, piece_hashes Vec<[u8;20]>
- [x] #2 Constructor validates piece_hashes.len() matches computed piece count from total_length and piece_length
- [x] #3 get_state(piece_index) returns reference to PieceState or error for invalid index
- [x] #4 start_piece(piece_index) transitions Missing/Failed to InProgress with empty BitVec
- [x] #5 All state transitions validate current state and return Result
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add PieceManager struct to 05-pieces.nw with fields: piece_states, piece_length, total_length, piece_hashes
2. Add constructor with validation (piece_hashes.len() matches computed piece count)
3. Add get_state method returning reference or error for invalid index
4. Add start_piece method to transition Missing/Failed to InProgress
5. Add block_received method to mark blocks
6. Add mark_verified and mark_failed methods
7. Add comprehensive tests
8. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented PieceManager struct in nw/05-pieces.nw with the following:

- PieceManager struct with fields: piece_states (Vec<PieceState>), piece_length (u32), total_length (u64), piece_hashes (Vec<[u8;20]>)
- Constructor validates piece_hashes.len() matches computed piece count using ceiling division
- get_state(piece_index) returns &PieceState or PieceError::InvalidPieceIndex
- start_piece(piece_index) transitions Missing/Failed to InProgress with correctly-sized BitVec
- mark_block_received(piece_index, block_index) marks blocks and returns Ok(bool)
- mark_verified(piece_index) and mark_failed(piece_index) for state transitions
- Helper methods: num_pieces(), piece_length(), total_length(), get_hash(), get_piece_size(), all_blocks_received(), verified_count(), pending_count(), is_complete()

All state transitions validate current state and return Result. Includes 14 comprehensive tests covering all methods and edge cases.
<!-- SECTION:NOTES:END -->
