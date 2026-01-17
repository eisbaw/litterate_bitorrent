---
id: task-0012
title: Implement piece availability tracking
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 16:38'
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
- [x] #1 PieceAvailability struct holds availability count per piece (Vec<u32>)
- [x] #2 new() initializes all counts to zero for given piece count
- [x] #3 add_peer_bitfield() increments counts for all pieces the peer has
- [x] #4 remove_peer_bitfield() decrements counts when peer disconnects
- [x] #5 add_piece() increments count for single piece (for have messages)
- [x] #6 get_availability(piece_index) returns current count
- [x] #7 Unit test: adding two peers with overlapping bitfields produces correct counts
- [x] #8 Unit test: removing a peer decrements counts correctly
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add PieceAvailability struct in a new noweb chunk
2. Implement new() to initialize Vec<u32> with zeros
3. Implement add_peer_bitfield() to increment counts
4. Implement remove_peer_bitfield() to decrement counts
5. Implement add_piece() for single piece increment
6. Implement get_availability() to return count for piece
7. Add unit tests for overlapping bitfields and removal
8. Integrate into module assembly
9. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Implementation Summary

Implemented PieceAvailability struct in nw/05-pieces.nw for tracking piece availability across the swarm.

### Key Design Decisions

- Uses Vec<u32> for O(1) lookup with minimal memory overhead
- Bitfield parsing follows BitTorrent protocol (MSB first within each byte)
- Saturating arithmetic prevents overflow/underflow bugs
- Includes rarest_piece() helper for rarest-first piece selection

### Files Modified

- nw/05-pieces.nw: Added PieceAvailability struct, implementation, and comprehensive tests

### Methods Implemented

- new(num_pieces) - Initialize with zero counts
- get_availability(piece_index) - Get count for specific piece
- add_peer_bitfield(&bitfield) - Increment counts from peer bitfield
- remove_peer_bitfield(&bitfield) - Decrement counts when peer disconnects
- add_piece(piece_index) - Increment single piece count (for have messages)
- iter() - Iterate over (piece_index, count) pairs
- rarest_piece() - Find piece with lowest non-zero availability

### Testing

- All unit tests pass (494 tests including 22 new availability tests)
- Doctests pass (41 tests including 7 new availability doctests)
- Tests cover overlapping bitfields, removal, edge cases, and bitfield parsing
<!-- SECTION:NOTES:END -->
