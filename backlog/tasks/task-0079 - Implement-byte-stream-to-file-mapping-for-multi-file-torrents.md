---
id: task-0079
title: Implement byte-stream-to-file mapping for multi-file torrents
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
labels:
  - phase4
  - disk
dependencies:
  - task-0017
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the mapping from the torrent's logical byte stream to physical file locations. A torrent represents all files as a single contiguous byte stream. For multi-file torrents, a piece may span multiple files. This task implements the calculation: given a byte offset and length in the stream, determine which file(s) it maps to and at what offset within each file. This is pure calculation - no I/O yet.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 FileSlice struct: file_index, file_offset, length
- [ ] #2 map_piece_to_files(piece_index, piece_length, files) returns Vec<FileSlice>
- [ ] #3 Handles piece fully within one file (common case)
- [ ] #4 Handles piece spanning two files (boundary case)
- [ ] #5 Handles piece spanning three or more files (rare but valid)
- [ ] #6 Unit test: 3 files of 100KB each, piece_length 64KB, verify piece 1 spans files 0 and 1
- [ ] #7 Unit test: piece fully within single file returns single FileSlice
<!-- AC:END -->
