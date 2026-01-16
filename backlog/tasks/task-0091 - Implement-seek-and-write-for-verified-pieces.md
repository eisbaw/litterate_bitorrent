---
id: task-0091
title: Implement seek and write for verified pieces
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
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
- [ ] #1 write_piece(piece_index, data, output_dir, files) writes verified piece data to correct location(s)
- [ ] #2 Uses file mapping to determine which files to write to
- [ ] #3 Opens files in write mode, seeks to correct offset, writes data slice
- [ ] #4 Handles piece spanning multiple files with multiple write operations
- [ ] #5 Returns DiskWriteError with file path and offset on failure
- [ ] #6 Unit test: write piece data to temp files, read back and verify content matches
- [ ] #7 Unit test: piece spanning two files is correctly split across both
<!-- AC:END -->
