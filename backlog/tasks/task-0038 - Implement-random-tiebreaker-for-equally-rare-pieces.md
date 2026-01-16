---
id: task-0038
title: Implement random tiebreaker for equally-rare pieces
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
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
- [ ] #1 select_piece_with_tiebreaker() returns single piece index from candidates with equal availability
- [ ] #2 Uses random selection among pieces with minimum availability count
- [ ] #3 Takes RNG as parameter for testability (dependency injection)
- [ ] #4 Unit test: with fixed seed RNG, selection is deterministic and reproducible
- [ ] #5 Unit test: with random RNG, selection distributes across equal-availability pieces over many runs
<!-- AC:END -->
