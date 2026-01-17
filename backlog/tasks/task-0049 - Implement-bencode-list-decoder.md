---
id: task-0049
title: Implement bencode list decoder
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 23:39'
labels:
  - bencode
  - parser
dependencies:
  - task-0011
  - task-0032
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Lists are encoded as l<values>e (e.g., l4:spami42ee). This is the first compound type requiring recursive parsing. Lists can contain any bencode value including nested lists and dicts.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Decodes empty list (le -> [])
- [x] #2 Decodes list with single element (li42e -> [Integer(42)])
- [x] #3 Decodes list with multiple elements of different types
- [x] #4 Decodes nested lists (lli1ei2eeli3eee)
- [x] #5 Returns error for unclosed list (missing e)
- [x] #6 Returns error for invalid element within list
- [x] #7 Unit tests cover all cases
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create decode_value function that dispatches based on first byte:
   - 'i' -> decode_integer
   - 'l' -> decode_list
   - 'd' -> decode_dict (placeholder, return error)
   - digit -> decode_bytes
2. Create decode_list function:
   - Verify starts with 'l'
   - Loop calling decode_value until 'e' is found
   - Handle errors from decode_value
3. Add comprehensive tests covering:
   - Empty list (le)
   - Single element (li42ee)
   - Multiple elements of different types
   - Nested lists
   - Error cases (unclosed, invalid element)
4. Update <<bencode.rs>> chunk to include new code
5. Run just tangle && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented bencode list decoder with the following changes to nw/01-bencode.nw:

## Summary
- Added decode_value() dispatcher function that routes to type-specific decoders based on first byte
- Added decode_list() function that recursively parses list elements until terminating 'e'
- Dictionary decoding is a placeholder (returns error) for future implementation

## Key design decisions
- decode_value dispatches: 'i'->integer, 'l'->list, 'd'->dict(placeholder), digit->bytes
- decode_list uses a loop calling decode_value for each element, enabling nested structures
- Error propagation is automatic via ? operator

## Tests added (20 new tests)
- Empty list, single element, multiple elements of different types
- Nested lists (including deeply nested)
- Error cases: unclosed list, invalid element, malformed nested element
- decode_value dispatcher tests for all type markers

## Files modified
- nw/01-bencode.nw: Added <<decode value>> and <<decode list>> chunks, updated <<bencode.rs>>
<!-- SECTION:NOTES:END -->
