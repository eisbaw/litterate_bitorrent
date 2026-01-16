---
id: task-0042
title: Implement PieceManager struct with state tracking
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
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
- [ ] #1 PieceManager struct holds: piece_states Vec<PieceState>, piece_length u32, total_length u64, piece_hashes Vec<[u8;20]>
- [ ] #2 Constructor validates piece_hashes.len() matches computed piece count from total_length and piece_length
- [ ] #3 get_state(piece_index) returns reference to PieceState or error for invalid index
- [ ] #4 start_piece(piece_index) transitions Missing/Failed to InProgress with empty BitVec
- [ ] #5 All state transitions validate current state and return Result
<!-- AC:END -->
