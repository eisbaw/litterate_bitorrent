---
id: task-0049
title: Implement bencode list decoder
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
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
- [ ] #1 Decodes empty list (le -> [])
- [ ] #2 Decodes list with single element (li42e -> [Integer(42)])
- [ ] #3 Decodes list with multiple elements of different types
- [ ] #4 Decodes nested lists (lli1ei2eeli3eee)
- [ ] #5 Returns error for unclosed list (missing e)
- [ ] #6 Returns error for invalid element within list
- [ ] #7 Unit tests cover all cases
<!-- AC:END -->
