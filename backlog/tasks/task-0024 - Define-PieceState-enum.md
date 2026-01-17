---
id: task-0024
title: Define PieceState enum
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 09:42'
labels:
  - phase4
  - foundation
dependencies:
  - task-0017
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define the PieceState enum that tracks the download state of each piece. This is the core state machine for piece management. The states are: Missing (not started), InProgress (some blocks received, tracked via BitVec), Verified (hash matched, written to disk), and Failed (hash mismatch, will retry). The InProgress variant carries a BitVec to track which blocks have been received, enabling resumption and detecting duplicate blocks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PieceState enum has variants: Missing, InProgress, Verified, Failed
- [x] #2 InProgress variant contains blocks_received: BitVec tracking which blocks are present
- [x] #3 Failed variant is distinct from Missing to allow tracking retry attempts
- [x] #4 State transitions documented: Missing->InProgress (first block), InProgress->Verified (hash ok), InProgress->Failed (hash mismatch), Failed->InProgress (retry)
- [x] #5 PieceState implements Clone and Debug
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented PieceState enum with four variants:
- Missing: Initial state, piece not started
- InProgress { blocks_received: BitVec<u8, Msb0> }: Actively downloading with block tracking
- Verified: Hash matched, written to disk
- Failed: Hash mismatch, needs retry

Key implementation details:
- BitVec (bitvec crate) used for efficient per-block tracking, enabling resumption and duplicate detection
- Failed distinct from Missing to support peer reputation tracking and retry limits
- State transitions documented with ASCII diagram in literate prose
- Helper methods: is_missing/in_progress/verified/failed, needs_download, new_in_progress, mark_block_received, blocks_received_count, total_blocks, all_blocks_received, has_block
- 15 comprehensive tests covering all states and methods

Files modified:
- nw/05-pieces.nw: Added PieceState subsection with literate explanation and tests
- src/pieces.rs: Generated via just tangle
<!-- SECTION:NOTES:END -->
