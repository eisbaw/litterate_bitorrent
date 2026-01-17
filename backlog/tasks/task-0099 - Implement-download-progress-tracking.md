---
id: task-0099
title: Implement download progress tracking
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 20:27'
labels:
  - phase4
  - state
dependencies:
  - task-0042
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement progress tracking for the download. Track the number of pieces in each state (Missing, InProgress, Verified, Failed) and provide methods to query progress. This enables progress display to the user and completion detection. Progress should be computed on demand from piece states, not stored separately (single source of truth principle). Also track bytes downloaded for more granular progress.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 PieceManager::progress() returns (verified_count, total_count)
- [x] #2 PieceManager::bytes_verified() returns total bytes in verified pieces
- [x] #3 PieceManager::is_complete() returns true when all pieces are Verified
- [x] #4 Progress is computed from piece_states, not cached separately
- [x] #5 Failed pieces are not counted as progress (they will be retried)
- [x] #6 Unit test: simulate piece verification sequence, verify progress updates correctly
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add progress() method returning (verified_count, total_count)
2. Add bytes_verified() method computing bytes in verified pieces (handling last piece size)
3. Add percent_complete() method returning percentage
4. Add documentation prose explaining the progress tracking design
5. Add unit tests for all new methods
6. Run just check && cargo test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented progress tracking in nw/05-pieces.nw.

Methods added to PieceManager:
- progress() -> (verified_count, total_count)
- bytes_verified() -> u64 (handles variable last piece size)
- percent_complete() -> f64 (0.0-100.0)
- is_complete() already existed

Design: Progress computed on demand from piece_states (single source of truth).
Failed pieces not counted as progress.

Tests added (9): zero start, verification increase, failed exclusion, complete detection, byte calculation, simulation sequence.

All 734 tests pass.
<!-- SECTION:NOTES:END -->
