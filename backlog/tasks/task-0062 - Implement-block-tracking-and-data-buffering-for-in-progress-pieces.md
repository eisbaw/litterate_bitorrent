---
id: task-0062
title: Implement block tracking and data buffering for in-progress pieces
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
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
- [ ] #1 InProgressPiece struct holds: data buffer (Vec<u8> pre-sized to piece size), blocks_received BitVec
- [ ] #2 receive_block(piece_index, offset, data) validates offset is block-aligned and within piece bounds
- [ ] #3 receive_block writes data to correct buffer position and marks block as received in BitVec
- [ ] #4 is_complete() returns true when all blocks in BitVec are set
- [ ] #5 Duplicate block (already received) is a no-op, not an error
- [ ] #6 Unit test: receive all blocks out of order, verify is_complete returns true
<!-- AC:END -->
