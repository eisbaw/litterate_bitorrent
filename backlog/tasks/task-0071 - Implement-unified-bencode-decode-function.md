---
id: task-0071
title: Implement unified bencode decode function
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-16 23:49'
labels:
  - bencode
  - parser
dependencies:
  - task-0061
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Top-level decode function that dispatches to the appropriate decoder based on the first byte. Provides the public API: decode(bytes) -> Result<BencodeValue, BencodeError>. Also ensures input is fully consumed.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 decode dispatches correctly based on first byte (i=integer, l=list, d=dict, 0-9=bytestring)
- [x] #2 Returns UnexpectedEof for empty input
- [x] #3 Returns InvalidFormat for unknown first byte
- [x] #4 Ensures entire input is consumed (trailing bytes cause error)
- [x] #5 Public function with clean API
- [x] #6 Unit tests verify dispatch to all four types
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add TrailingData error variant to BencodeError enum
2. Implement decode function that wraps decode_value and checks for complete input consumption
3. Add unit tests for:
   - dispatch to integer
   - dispatch to bytestring
   - dispatch to list
   - dispatch to dict
   - empty input -> UnexpectedEof
   - unknown first byte -> InvalidFormat
   - trailing bytes -> TrailingData
4. Update module structure to include new chunk
5. Run tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added unified decode() function to bencode module:

- Added TrailingData(usize) error variant to BencodeError
- Implemented decode(input: &[u8]) -> Result<BencodeValue, BencodeError>
- decode wraps decode_value and enforces complete input consumption
- Added 12 new unit tests covering dispatch to all 4 types, empty input, unknown bytes, and trailing data rejection
- All 90 tests pass, lint clean

Files modified:
- nw/01-bencode.nw (source of truth)
- src/bencode.rs (generated via just tangle)
<!-- SECTION:NOTES:END -->
