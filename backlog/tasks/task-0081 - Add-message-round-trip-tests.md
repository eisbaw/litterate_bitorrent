---
id: task-0081
title: Add message round-trip tests
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
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
- [ ] #1 Round-trip test for each message type: to_bytes then from_bytes yields identical message
- [ ] #2 Test Bitfield with 0, 1, 100, 1000 pieces (varying lengths)
- [ ] #3 Test Have with piece_index=0, piece_index=u32::MAX
- [ ] #4 Test Request/Cancel with begin=0 and begin at piece boundary
- [ ] #5 Test Piece with empty data, BLOCK_SIZE data, and data > BLOCK_SIZE
- [ ] #6 All tests pass with no panics or assertion failures
<!-- AC:END -->
