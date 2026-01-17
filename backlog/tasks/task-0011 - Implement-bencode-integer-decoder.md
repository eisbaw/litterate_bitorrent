---
id: task-0011
title: Implement bencode integer decoder
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 23:26'
labels:
  - bencode
  - parser
dependencies:
  - task-0003
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
First step in recursive descent parsing. Integers are encoded as i<number>e (e.g., i42e, i-3e, i0e). This is the simplest bencode type and validates the parsing approach before tackling more complex types.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Decodes positive integers correctly (i42e -> 42)
- [x] #2 Decodes negative integers correctly (i-3e -> -3)
- [x] #3 Decodes zero correctly (i0e -> 0)
- [x] #4 Rejects leading zeros (i03e is invalid per spec)
- [x] #5 Rejects i-0e (negative zero is invalid)
- [x] #6 Returns descriptive error for malformed input (missing i, missing e, non-numeric content)
- [x] #7 Unit tests cover all cases above
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add prose section explaining integer parsing strategy
2. Implement decode_integer function:
   - Check for 'i' prefix
   - Find 'e' terminator
   - Extract number string
   - Validate: no leading zeros, no -0
   - Parse to i64
   - Advance position
3. Add unit tests covering:
   - Positive integers (i42e -> 42)
   - Negative integers (i-3e -> -3)
   - Zero (i0e -> 0)
   - Reject leading zeros (i03e)
   - Reject -0 (i-0e)
   - Malformed input errors
4. Run tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented bencode integer decoder (decode_integer function) with comprehensive validation:

- Added prose section explaining parsing strategy in literate style
- Implemented decode_integer(input: &[u8], pos: &mut usize) -> Result<i64, BencodeError>
- Validates: no leading zeros (except i0e), no negative zero (i-0e)
- Returns descriptive errors for all malformed input cases
- 15 unit tests covering all acceptance criteria

Modified files:
- nw/01-bencode.nw: Added decode integer chunk and bencode tests chunk
- src/bencode.rs: Generated from tangle
<!-- SECTION:NOTES:END -->
