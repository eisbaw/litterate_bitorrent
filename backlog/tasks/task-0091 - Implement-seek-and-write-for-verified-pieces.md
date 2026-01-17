---
id: task-0091
title: Implement seek and write for verified pieces
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 17:23'
labels:
  - phase4
  - disk
dependencies:
  - task-0086
  - task-0079
  - task-0072
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the disk writer that writes verified piece data to the correct file location(s). Using the file mapping from the previous task, seek to the correct position in each file and write the corresponding slice of piece data. For pieces spanning multiple files, this involves multiple seek+write operations. Uses explicit seek+write (not mmap) for pedagogical clarity as noted in the PRD. Handle disk full gracefully.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 write_piece(piece_index, data, output_dir, files) writes verified piece data to correct location(s)
- [x] #2 Uses file mapping to determine which files to write to
- [x] #3 Opens files in write mode, seeks to correct offset, writes data slice
- [x] #4 Handles piece spanning multiple files with multiple write operations
- [x] #5 Returns DiskWriteError with file path and offset on failure
- [x] #6 Unit test: write piece data to temp files, read back and verify content matches
- [x] #7 Unit test: piece spanning two files is correctly split across both
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add FileSlice struct to disk module - represents a slice of a file to write to
2. Implement map_piece_to_files(piece_index, piece_length, total_length, files) - maps piece to file slices
3. Add DiskError::WriteError variant for write failures
4. Implement write_piece(piece_index, data, piece_length, total_length, output_dir, files)
5. Handle single-file case (common, simple) and multi-file case (piece spanning files)
6. Write unit tests for single-file writes and multi-file spanning writes
7. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented seek and write for verified pieces in nw/08-disk.nw.

Key components added:
- FileSlice struct: represents a slice of data to write to a specific file at a specific offset
- map_piece_to_files(): maps a piece index to the file slices it spans, handling single-file and multi-file torrents
- write_piece(): writes verified piece data to correct file locations using explicit seek+write operations
- DiskError variants: SeekError and WriteError with path and offset context

Design decisions:
- Uses explicit seek+write approach (not mmap) for pedagogical clarity per PRD
- Handles pieces spanning multiple files by splitting data across seek+write ops
- Does not call sync_data/sync_all after writes (performance vs durability tradeoff)
- Files must already exist (via create_output_files) before writing

Tests added (11 new tests):
- map_piece_single_file_first_piece, middle_piece, last_piece_shorter
- map_piece_spanning_two_files, spanning_three_files, entirely_in_second_file
- write_piece_single_file, single_file_middle, spanning_two_files, last_piece_shorter
- write_piece_error_includes_path

Verified with: just check && just test (583 tests pass)
Note: Pre-existing clippy warnings in pieces.rs are unrelated to this task.
<!-- SECTION:NOTES:END -->
