---
id: task-0062
title: Implement block tracking and data buffering for in-progress pieces
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 17:00'
labels:
  - phase4
  - state
dependencies:
  - task-0024
  - task-0017
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the data structures and logic to buffer incoming blocks for pieces that are in progress. Each in-progress piece needs a byte buffer to accumulate block data and a BitVec to track which blocks have been received. When a block arrives, it must be validated (correct piece, valid offset, expected size) and written to the correct position in the buffer. Duplicate blocks should be handled gracefully (no-op). This is the accumulation phase before verification.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 InProgressPiece struct holds: data buffer (Vec<u8> pre-sized to piece size), blocks_received BitVec
- [x] #2 receive_block(piece_index, offset, data) validates offset is block-aligned and within piece bounds
- [x] #3 receive_block writes data to correct buffer position and marks block as received in BitVec
- [x] #4 is_complete() returns true when all blocks in BitVec are set
- [x] #5 Duplicate block (already received) is a no-op, not an error
- [x] #6 Unit test: receive all blocks out of order, verify is_complete returns true
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add InProgressPiece struct holding data buffer (Vec<u8>) and blocks_received BitVec
2. Implement receive_block() with offset validation and block-aligned check
3. Implement is_complete() checking if all blocks received
4. Implement get_data() to return assembled piece data
5. Add comprehensive unit tests including out-of-order block reception
6. Add chunk to module assembly
7. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented InProgressPiece struct for buffering incoming blocks during piece download.

Key implementation details:
- Pre-allocated Vec<u8> data buffer sized to piece_size
- BitVec tracking received blocks (same approach as PieceState::InProgress)
- receive_block() validates: block alignment (multiple of BLOCK_SIZE), bounds checking, and data length
- Duplicate blocks return Ok(false) as no-op, not an error
- is_complete() checks if all bits are set in blocks_received
- take_data() consumes the struct and returns assembled piece data
- reset() method for reuse after verification failure

The design separates concerns: PieceState tracks abstract state (Missing/InProgress/Verified/Failed), while InProgressPiece holds actual byte data. This keeps PieceManager lightweight.

15 unit tests covering: initialization, out-of-order block reception, duplicate handling, validation errors, reset, and the specific AC#6 test for receiving all blocks in scrambled order.
<!-- SECTION:NOTES:END -->
