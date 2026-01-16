---
id: task-0086
title: Implement file creation with parent directories and sparse allocation
status: To Do
assignee: []
created_date: '2026-01-16 21:32'
labels:
  - phase4
  - disk
dependencies:
  - task-0079
  - task-0008
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement file creation for torrent output. Before writing piece data, the target files must exist. This involves creating parent directories if needed and creating the files as sparse files (no pre-allocation - the OS handles actual disk allocation on write). Sparse file creation means setting the file size but not writing data, so disk space is only consumed as data is written. This is idempotent - calling it multiple times is safe.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 create_output_files(output_dir, files) creates all files from metainfo file list
- [ ] #2 Creates parent directories recursively (like mkdir -p)
- [ ] #3 Creates files as sparse files with correct size (seek to end, truncate)
- [ ] #4 Operation is idempotent - existing files are not overwritten or truncated
- [ ] #5 Returns DiskWriteError with path context on failure
- [ ] #6 Unit test: create files in temp directory, verify sizes are correct
<!-- AC:END -->
