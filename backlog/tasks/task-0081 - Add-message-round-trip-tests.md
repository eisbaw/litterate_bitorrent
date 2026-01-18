---
id: task-0081
title: Add message round-trip tests
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-18 01:59'
labels:
  - phase-3
  - protocol
  - messages
  - testing
dependencies:
  - task-0064
  - task-0073
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Verify that all message types survive serialization and parsing without data loss. This catches encoding/decoding asymmetries and ensures protocol correctness. Test edge cases like empty bitfields, maximum block sizes, and large piece indices.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Round-trip test for each message type: to_bytes then from_bytes yields identical message
- [x] #2 Test Bitfield with 0, 1, 100, 1000 pieces (varying lengths)
- [x] #3 Test Have with piece_index=0, piece_index=u32::MAX
- [x] #4 Test Request/Cancel with begin=0 and begin at piece boundary
- [x] #5 Test Piece with empty data, BLOCK_SIZE data, and data > BLOCK_SIZE
- [x] #6 All tests pass with no panics or assertion failures
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add new noweb chunk <<message round-trip tests>> for comprehensive round-trip tests
2. Include tests for all message types with edge cases:
   - Bitfield: 0, 1, 100, 1000 pieces
   - Have: piece_index=0, piece_index=u32::MAX
   - Request/Cancel: begin=0 and begin at piece boundary (BLOCK_SIZE multiples)
   - Piece: empty data, BLOCK_SIZE data, data > BLOCK_SIZE
3. Add chunk reference to <<message tests>>
4. Add prose explaining the round-trip test strategy
5. Verify tangle, lint, and test pass
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added comprehensive round-trip tests:
- 32 tests covering all message types
- Bitfield: 0, 1, 8, 100, 1000 pieces
- Have: piece_index=0 and u32::MAX
- Request/Cancel: begin=0 and piece boundaries
- Piece: empty, BLOCK_SIZE, larger data
- All 845 tests pass
<!-- SECTION:NOTES:END -->
