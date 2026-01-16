---
id: task-0032
title: Implement bencode byte string decoder
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
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
- [ ] #1 Decodes simple byte strings (4:spam -> [115, 112, 97, 109])
- [ ] #2 Decodes empty byte strings (0: -> [])
- [ ] #3 Handles binary data (non-UTF8 bytes)
- [ ] #4 Returns error for length exceeding available input
- [ ] #5 Returns error for negative or non-numeric length prefix
- [ ] #6 Returns error for missing colon separator
- [ ] #7 Unit tests cover all cases
<!-- AC:END -->
