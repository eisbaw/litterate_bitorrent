---
id: task-0099
title: Implement download progress tracking
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
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
- [ ] #1 PieceManager::progress() returns (verified_count, total_count)
- [ ] #2 PieceManager::bytes_verified() returns total bytes in verified pieces
- [ ] #3 PieceManager::is_complete() returns true when all pieces are Verified
- [ ] #4 Progress is computed from piece_states, not cached separately
- [ ] #5 Failed pieces are not counted as progress (they will be retried)
- [ ] #6 Unit test: simulate piece verification sequence, verify progress updates correctly
<!-- AC:END -->
