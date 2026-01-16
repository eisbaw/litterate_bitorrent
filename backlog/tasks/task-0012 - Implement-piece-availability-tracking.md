---
id: task-0012
title: Implement piece availability tracking
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - phase-5
  - strategy
  - availability
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Track how many peers have each piece across the swarm. This is prerequisite for rarest-first selection - we need to know piece frequency before we can select the rarest. Maintain a count array indexed by piece_index, updated when peers send bitfield or have messages.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PieceAvailability struct holds availability count per piece (Vec<u32>)
- [ ] #2 new() initializes all counts to zero for given piece count
- [ ] #3 add_peer_bitfield() increments counts for all pieces the peer has
- [ ] #4 remove_peer_bitfield() decrements counts when peer disconnects
- [ ] #5 add_piece() increments count for single piece (for have messages)
- [ ] #6 get_availability(piece_index) returns current count
- [ ] #7 Unit test: adding two peers with overlapping bitfields produces correct counts
- [ ] #8 Unit test: removing a peer decrements counts correctly
<!-- AC:END -->
