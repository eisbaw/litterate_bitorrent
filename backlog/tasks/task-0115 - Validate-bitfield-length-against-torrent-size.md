---
id: task-0115
title: Validate bitfield length against torrent size
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:38'
updated_date: '2026-01-17 21:55'
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
- [x] #1 bitfield_length(num_pieces: u32) -> usize returns ceil(num_pieces/8)
- [x] #2 validate_bitfield(bits: &[u8], num_pieces: u32) -> Result<(), Error> checks length and spare bits
- [x] #3 Returns Error::BitfieldWrongLength if length mismatch
- [x] #4 Returns Error::InvalidMessage if spare bits are set (indicates buggy client)
- [x] #5 Unit tests: 0 pieces, 1 piece, 8 pieces, 9 pieces, 1000 pieces
- [x] #6 Unit tests: spare bits zero vs nonzero
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add bitfield_length function to compute ceil(num_pieces/8)
2. Add validate_bitfield function to check length and spare bits
3. Add unit tests for edge cases: 0, 1, 8, 9, 1000 pieces
4. Add unit tests for spare bits validation
5. Update module structure to include new chunks
6. Run just lint and just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented bitfield validation for the peer wire protocol:

- Added `bitfield_length(num_pieces: u32) -> usize` function that computes ceil(num_pieces/8)
- Added `validate_bitfield(bits: &[u8], num_pieces: u32) -> Result<(), PeerError>` that:
  - Returns BitfieldWrongLength if length mismatch
  - Returns InvalidMessage if spare bits are set
- Comprehensive unit tests covering edge cases:
  - 0, 1, 8, 9, 1000 pieces
  - Spare bits validation (zero vs nonzero)
  - Exact multiples of 8 (no spare bits)
- Literate documentation explaining the bit layout and validation logic

Files modified:
- nw/04-messages.nw: Added <<bitfield validation>> and <<bitfield validation tests>> chunks
<!-- SECTION:NOTES:END -->
