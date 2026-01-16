---
id: task-0104
title: Implement single-file torrent metainfo parsing
status: To Do
assignee: []
created_date: '2026-01-16 21:34'
labels:
  - metainfo
  - parser
dependencies:
  - task-0102
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse .torrent files in single-file format. Extract announce URL, name, piece length, and piece hashes from bencoded dict. Single-file format has length field directly in info dict rather than files array.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Parses announce URL from top-level dict
- [ ] #2 Extracts name from info dict
- [ ] #3 Extracts piece_length from info dict
- [ ] #4 Parses pieces byte string into Vec of 20-byte SHA1 hashes
- [ ] #5 Returns clear error if pieces length is not multiple of 20
- [ ] #6 Extracts length field for single-file format
- [ ] #7 Creates single FileInfo entry with name and length
- [ ] #8 Computes total_length correctly
- [ ] #9 Unit test parses hand-crafted minimal single-file torrent
<!-- AC:END -->
