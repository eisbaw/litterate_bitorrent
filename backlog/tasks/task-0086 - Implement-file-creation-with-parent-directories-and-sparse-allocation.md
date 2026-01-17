---
id: task-0086
title: Implement file creation with parent directories and sparse allocation
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:32'
updated_date: '2026-01-17 17:17'
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
- [x] #1 create_output_files(output_dir, files) creates all files from metainfo file list
- [x] #2 Creates parent directories recursively (like mkdir -p)
- [x] #3 Creates files as sparse files with correct size (seek to end, truncate)
- [x] #4 Operation is idempotent - existing files are not overwritten or truncated
- [x] #5 Returns DiskWriteError with path context on failure
- [x] #6 Unit test: create files in temp directory, verify sizes are correct
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/08-disk.nw with literate prose explaining file creation for torrent downloads
2. Define DiskError type with path context for failures
3. Implement create_output_files(output_dir, files) function:
   - Create parent directories recursively (mkdir -p style)
   - Create sparse files using seek + set_len
   - Handle idempotency (skip existing files with correct size)
4. Update nw/00-main.nw to include disk module in lib.rs
5. Update justfile with notangle command for disk.rs
6. Write comprehensive unit tests:
   - Test basic file creation in temp directory
   - Test parent directory creation
   - Test sparse file allocation (correct size)
   - Test idempotency (existing files not overwritten)
   - Test error handling for permission denied
7. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented disk I/O module for file creation in nw/08-disk.nw.

Key components:
- DiskError enum with path context for: CreateDirectory, CreateFile, SetFileSize, Metadata
- create_output_files(output_dir, files) function that:
  - Creates parent directories recursively using fs::create_dir_all
  - Creates sparse files using OpenOptions + set_len
  - Is idempotent: existing files with correct size are not modified
  - Preserves existing file content (does not truncate)

Files modified:
- nw/08-disk.nw: New literate file with disk module implementation
- nw/00-main.nw: Added disk module to lib.rs exports
- justfile: Added notangle command for disk.rs

Tests cover:
- Single file creation with correct sparse size
- Parent directory creation (deep nested paths)
- Multiple file creation
- Idempotency (existing files unchanged)
- Zero-length files
- Sparse file allocation (1 GB file created instantly)
- Error context includes path information
- Empty file list handling

Verified with: just check && just test (572 tests pass)
<!-- SECTION:NOTES:END -->
