---
id: task-0079
title: Implement byte-stream-to-file mapping for multi-file torrents
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-17 17:26'
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
- [x] #1 FileSlice struct: file_index, file_offset, length
- [x] #2 map_piece_to_files(piece_index, piece_length, files) returns Vec<FileSlice>
- [x] #3 Handles piece fully within one file (common case)
- [x] #4 Handles piece spanning two files (boundary case)
- [x] #5 Handles piece spanning three or more files (rare but valid)
- [x] #6 Unit test: 3 files of 100KB each, piece_length 64KB, verify piece 1 spans files 0 and 1
- [x] #7 Unit test: piece fully within single file returns single FileSlice
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Task-0091 had already implemented the core functionality for byte-stream-to-file mapping:
- FileSlice struct with file_index, file_offset, length fields
- map_piece_to_files() function that maps piece index to file slices
- Tests for single-file, two-file spanning, and three-file spanning cases

This task added two explicit acceptance criteria tests:
1. map_piece_100kb_files_64kb_pieces: Tests exactly 3 files of 100KB each with 64KB pieces, verifying piece 1 spans files 0 and 1
2. map_piece_single_file_returns_single_slice: Explicitly tests that a piece within a single file returns exactly one FileSlice

All 585 unit tests pass. No changes to core logic were needed since the implementation in task-0091 was complete.
<!-- SECTION:NOTES:END -->
