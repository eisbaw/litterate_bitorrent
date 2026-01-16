---
id: task-0071
title: Implement unified bencode decode function
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
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
- [ ] #1 decode dispatches correctly based on first byte (i=integer, l=list, d=dict, 0-9=bytestring)
- [ ] #2 Returns UnexpectedEof for empty input
- [ ] #3 Returns InvalidFormat for unknown first byte
- [ ] #4 Ensures entire input is consumed (trailing bytes cause error)
- [ ] #5 Public function with clean API
- [ ] #6 Unit tests verify dispatch to all four types
<!-- AC:END -->
