---
id: task-0078
title: Implement bencode encoder
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
labels:
  - bencode
  - encoder
dependencies:
  - task-0071
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Encoder is necessary for info hash computation - we must re-encode the info dict to compute its SHA1. Encoding is simpler than decoding as we just traverse the BencodeValue tree. BTreeMap ensures canonical dict key ordering.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Encodes Integer correctly (42 -> i42e)
- [ ] #2 Encodes negative integers and zero
- [ ] #3 Encodes Bytes correctly (vec -> length:bytes)
- [ ] #4 Encodes empty byte strings
- [ ] #5 Encodes List recursively
- [ ] #6 Encodes Dict with keys in sorted byte order (BTreeMap handles this)
- [ ] #7 Encodes nested structures correctly
- [ ] #8 Unit tests cover all types
<!-- AC:END -->
