---
id: task-0019
title: Implement rarest-first piece selection
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 16:43'
labels:
  - phase-5
  - strategy
  - selection
dependencies:
  - task-0012
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Core algorithm for BitTorrent piece selection. Rarest-first prioritizes pieces with lowest availability to improve swarm health and download speed. Must filter out pieces we already have or are in progress, then select from the remaining based on availability count.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 select_rarest() returns piece indices sorted by availability (ascending)
- [x] #2 Excludes pieces already verified (Verified state)
- [x] #3 Excludes pieces currently in progress (InProgress state)
- [x] #4 Returns empty vec if no pieces are available from the given peer
- [x] #5 Unit test: with availabilities [3, 1, 2], returns order [1, 2, 0]
- [x] #6 Unit test: filters out already-completed pieces correctly
- [x] #7 Unit test: returns empty when peer has no pieces we need
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add new section "Rarest-First Piece Selection" after PieceAvailability
2. Implement select_rarest() function taking PieceManager ref, PieceAvailability ref, and peer bitfield
3. Filter out Verified and InProgress pieces
4. Filter to only pieces the peer has
5. Sort remaining by availability count (ascending)
6. Return Vec<u32> of piece indices
7. Add comprehensive tests
8. Update module assembly to include new code
9. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented rarest-first piece selection in nw/05-pieces.nw:

- Added new subsection "Rarest-First Piece Selection" with literate prose explaining the algorithm
- Implemented select_rarest() function that takes PieceManager, PieceAvailability, and peer bitfield
- Function returns Vec<u32> of piece indices sorted by availability (ascending)
- Filters out Verified pieces (already have)
- Filters out InProgress pieces (already downloading)
- Filters to only pieces the peer has (bitfield check)
- Failed pieces remain selectable for re-download
- Tie-breaking by piece index for deterministic behavior
- Added 13 comprehensive unit tests covering:
  - Basic sorting by availability
  - Verified piece exclusion
  - InProgress piece exclusion
  - Peer bitfield filtering
  - Empty results when peer has nothing we need
  - Partial bitfield handling
  - Tie-breaking behavior
  - Failed piece inclusion
  - Edge cases (empty torrent, single piece)

All tests pass: just check && just test
<!-- SECTION:NOTES:END -->
