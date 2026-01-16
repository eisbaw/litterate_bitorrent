---
id: task-0115
title: Validate bitfield length against torrent size
status: To Do
assignee: []
created_date: '2026-01-16 21:38'
labels:
  - phase-3
  - protocol
  - validation
dependencies:
  - task-0052
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When a peer sends a bitfield, its length must match the expected number of pieces. The bitfield has ceil(num_pieces/8) bytes. Spare bits in the last byte must be zero. This validation prevents buffer overruns and detects protocol violations from misbehaving peers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 bitfield_length(num_pieces: u32) -> usize returns ceil(num_pieces/8)
- [ ] #2 validate_bitfield(bits: &[u8], num_pieces: u32) -> Result<(), Error> checks length and spare bits
- [ ] #3 Returns Error::BitfieldWrongLength if length mismatch
- [ ] #4 Returns Error::InvalidMessage if spare bits are set (indicates buggy client)
- [ ] #5 Unit tests: 0 pieces, 1 piece, 8 pieces, 9 pieces, 1000 pieces
- [ ] #6 Unit tests: spare bits zero vs nonzero
<!-- AC:END -->
