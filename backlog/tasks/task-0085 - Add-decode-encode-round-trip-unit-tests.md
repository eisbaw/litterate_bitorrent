---
id: task-0085
title: Add decode-encode round-trip unit tests
status: Done
assignee: []
created_date: '2026-01-16 21:32'
updated_date: '2026-01-16 23:56'
labels:
  - bencode
  - testing
dependencies:
  - task-0078
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Verify that encode(decode(input)) == input for valid bencode and decode(encode(value)) == value. This is the fundamental correctness property. Tests should cover edge cases like empty containers, nested structures, and boundary values.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Test integer round-trip including i0e, i-1e, large values near i64 boundaries
- [x] #2 Test byte string round-trip including empty and binary data
- [x] #3 Test list round-trip including empty and nested
- [x] #4 Test dict round-trip including empty and nested
- [x] #5 Test complex nested structure (dict containing list containing dict)
- [x] #6 All tests pass
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Round-trip tests already added in task-0078 encoder implementation.
<!-- SECTION:NOTES:END -->
