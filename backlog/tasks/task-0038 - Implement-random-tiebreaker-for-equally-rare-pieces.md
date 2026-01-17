---
id: task-0038
title: Implement random tiebreaker for equally-rare pieces
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 16:48'
labels:
  - phase-5
  - strategy
  - selection
dependencies:
  - task-0019
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When multiple pieces have the same availability count, randomly select among them to avoid herding (all peers requesting the same piece). This is critical for swarm health - without randomization, all leechers would compete for the same rare piece, creating a bottleneck.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 select_piece_with_tiebreaker() returns single piece index from candidates with equal availability
- [x] #2 Uses random selection among pieces with minimum availability count
- [x] #3 Takes RNG as parameter for testability (dependency injection)
- [x] #4 Unit test: with fixed seed RNG, selection is deterministic and reproducible
- [x] #5 Unit test: with random RNG, selection distributes across equal-availability pieces over many runs
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add literate prose explaining the need for random tiebreaking
2. Implement select_piece_with_tiebreaker() function
3. Write unit tests for deterministic (fixed seed) and stochastic behavior
4. Add to module assembly
5. Run just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented random tiebreaker for equally-rare pieces in the pieces module.

Key changes:
- Added select_piece_with_tiebreaker() function that takes candidates (piece_index, availability_count) and an RNG
- Uses dependency injection pattern: RNG parameter enables both seeded (testable) and production usage
- Algorithm: finds minimum availability count, collects all pieces with that count, randomly selects one

Tests added:
- tiebreaker_returns_none_for_empty_candidates
- tiebreaker_returns_single_piece_when_one_candidate
- tiebreaker_selects_minimum_availability_piece
- tiebreaker_with_fixed_seed_is_deterministic
- tiebreaker_result_is_from_minimum_set
- tiebreaker_distributes_across_equal_pieces_over_many_runs (statistical)
- tiebreaker_handles_all_same_availability
- tiebreaker_handles_large_availability_counts

All 515 tests pass including 8 new tiebreaker tests.
<!-- SECTION:NOTES:END -->
