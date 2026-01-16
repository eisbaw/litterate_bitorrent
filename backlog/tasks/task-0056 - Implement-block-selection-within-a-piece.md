---
id: task-0056
title: Implement block selection within a piece
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
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
- [ ] #1 select_next_blocks() returns list of (offset, length) pairs for unrequested blocks in a piece
- [ ] #2 Respects BLOCK_SIZE constant (16384 bytes)
- [ ] #3 Calculates correct length for final block when piece_length is not multiple of BLOCK_SIZE
- [ ] #4 Skips blocks already marked as received in piece state
- [ ] #5 Skips blocks already in-flight (pending request)
- [ ] #6 Unit test: 64KB piece produces 4 blocks of 16KB each
- [ ] #7 Unit test: 50KB piece produces 3 blocks of 16KB and 1 block of 2KB
- [ ] #8 Unit test: skips already-received blocks correctly
<!-- AC:END -->
