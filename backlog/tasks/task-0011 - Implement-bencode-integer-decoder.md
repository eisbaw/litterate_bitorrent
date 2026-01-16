---
id: task-0011
title: Implement bencode integer decoder
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
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
- [ ] #1 Decodes positive integers correctly (i42e -> 42)
- [ ] #2 Decodes negative integers correctly (i-3e -> -3)
- [ ] #3 Decodes zero correctly (i0e -> 0)
- [ ] #4 Rejects leading zeros (i03e is invalid per spec)
- [ ] #5 Rejects i-0e (negative zero is invalid)
- [ ] #6 Returns descriptive error for malformed input (missing i, missing e, non-numeric content)
- [ ] #7 Unit tests cover all cases above
<!-- AC:END -->
