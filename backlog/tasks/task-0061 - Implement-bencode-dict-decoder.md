---
id: task-0061
title: Implement bencode dict decoder
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
labels:
  - bencode
  - parser
dependencies:
  - task-0049
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Dictionaries are encoded as d<key><value>...e where keys must be byte strings in sorted order. BTreeMap ensures canonical ordering for info hash computation. This completes the recursive descent decoder.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Decodes empty dict (de -> {})
- [ ] #2 Decodes dict with single key-value pair
- [ ] #3 Decodes dict with multiple entries
- [ ] #4 Decodes nested dicts (dict containing dict)
- [ ] #5 Rejects non-string keys with clear error
- [ ] #6 Handles keys with different sorting than ASCII order (raw byte comparison)
- [ ] #7 Returns error for unclosed dict
- [ ] #8 Returns error for duplicate keys
- [ ] #9 Unit tests cover all cases
<!-- AC:END -->
