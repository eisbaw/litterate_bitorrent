---
id: task-0008
title: Define piece error types
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 09:38'
labels:
  - phase4
  - foundation
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create error types for piece management operations. These errors need clear context for debugging hash mismatches, disk failures, and geometry violations. Following fail-fast principles, each error must include enough information to diagnose the problem without additional logging.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PieceHashMismatch error includes piece index, expected hash (hex), and computed hash (hex)
- [x] #2 InvalidPieceIndex error includes the invalid index and the valid range
- [x] #3 InvalidBlockOffset error includes piece index, offset, and piece size
- [x] #4 DiskWriteError wraps std::io::Error with file path and byte offset context
- [x] #5 All errors implement std::error::Error and Display with human-readable messages
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/05-pieces.nw with LaTeX preamble and section intro
2. Define PieceError enum with all required variants (PieceHashMismatch, InvalidPieceIndex, InvalidBlockOffset, DiskWriteError)
3. Implement thiserror derives and Display traits
4. Add literate prose explaining each error type and recovery strategies
5. Add tests for error display and construction
6. Update nw/00-main.nw to add pub mod pieces
7. Update justfile to tangle pieces.rs
8. Verify with nix-shell --run "just check && just test"
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented piece error types in nw/05-pieces.nw following the established literate programming pattern.

Changes:
- Created nw/05-pieces.nw with LaTeX preamble and literate prose explaining piece management error handling
- Defined PieceError enum with four variants:
  - PieceHashMismatch: includes piece_index, expected_hash (hex), computed_hash (hex)
  - InvalidPieceIndex: includes invalid index and valid range (num_pieces)
  - InvalidBlockOffset: includes piece_index, offset, and piece_size
  - DiskWriteError: wraps std::io::Error with path and byte_offset context
- All errors derive Debug, thiserror::Error and implement Display
- Added helper constructors (hash_mismatch, invalid_index, invalid_offset, disk_write)
- Implemented hash_to_hex helper to avoid external hex crate dependency
- Updated nw/00-main.nw to add pub mod pieces
- Updated justfile to tangle pieces.rs
- Added comprehensive tests for display formatting, trait implementations, and helpers
- Verified with just check && just test (352 tests pass)
<!-- SECTION:NOTES:END -->
