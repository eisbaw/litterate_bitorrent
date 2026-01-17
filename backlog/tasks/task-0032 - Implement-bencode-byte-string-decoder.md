---
id: task-0032
title: Implement bencode byte string decoder
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 23:33'
labels:
  - bencode
  - parser
dependencies:
  - task-0003
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Byte strings are encoded as length:content (e.g., 4:spam). Unlike integers, the content is raw bytes not text. This handles the second primitive bencode type and introduces length-prefixed parsing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Decodes simple byte strings (4:spam -> [115, 112, 97, 109])
- [x] #2 Decodes empty byte strings (0: -> [])
- [x] #3 Handles binary data (non-UTF8 bytes)
- [x] #4 Returns error for length exceeding available input
- [x] #5 Returns error for negative or non-numeric length prefix
- [x] #6 Returns error for missing colon separator
- [x] #7 Unit tests cover all cases
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read existing code structure and understand the pattern
2. Add <<decode bytes>> chunk with prose explaining byte string format
3. Implement decode_bytes function following the same pattern as decode_integer
4. Add comprehensive tests covering all acceptance criteria in the <<bencode tests>> chunk
5. Update <<bencode.rs>> to include the new chunk
6. Run just tangle to generate Rust code
7. Run just test to verify all tests pass
8. Run just lint to check for code quality issues
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented bencode byte string decoder in nw/01-bencode.nw:

- Added <<decode bytes>> chunk with decode_bytes() function
- Function parses length-prefixed format: <digits>:<content>
- Validates: no leading zeros, all digits, sufficient input after colon
- Handles arbitrary binary data (non-UTF8 bytes)
- Uses checked_add to prevent arithmetic overflow
- Added 16 unit tests covering all acceptance criteria:
  - Simple strings, empty strings, binary data
  - Error cases: length overflow, negative/non-numeric length, missing colon
  - Edge cases: colon in content, digits in content, non-zero starting position

All 31 tests pass (16 new byte string tests + 15 existing integer tests).
<!-- SECTION:NOTES:END -->
