---
id: task-0108
title: Implement multi-file torrent metainfo parsing
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:35'
updated_date: '2026-01-17 00:26'
labels:
  - metainfo
  - parser
dependencies:
  - task-0104
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extend metainfo parsing for multi-file format. Multi-file torrents have a files array in info dict, where each entry has length and path fields. Path is a list of path components. Total length is sum of all file lengths.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Detects multi-file format (files array present, no length field)
- [x] #2 Parses files array correctly
- [x] #3 Extracts length from each file entry
- [x] #4 Joins path components with OS path separator
- [x] #5 Handles nested directories in path
- [x] #6 Computes total_length as sum of all file lengths
- [x] #7 Returns error for malformed files array
- [x] #8 Unit test with hand-crafted multi-file torrent dict
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add InvalidMultiFile error variant to MetainfoError enum
2. Add get_list helper function for extracting lists from BencodeValue
3. Implement parse_multi_file function that:
   - Extracts files array from info dict
   - Parses each file entry (length, path)
   - Joins path components into PathBuf
   - Computes total_length as sum of file lengths
4. Update parse function to detect multi-file format (files key present, no length key)
5. Add comprehensive unit tests for multi-file parsing
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented multi-file torrent metainfo parsing in /home/mpedersen/topics/litterate_bitorrent/nw/02-metainfo.nw.

Changes:
- Added InvalidMultiFile error variant to MetainfoError for malformed files arrays
- Added get_list helper function for extracting lists from BencodeValue
- Implemented parse_multi_file function that:
  - Validates files array is non-empty
  - Parses each file entry dictionary for length and path
  - Joins path components into PathBuf with torrent name as root directory
  - Validates path components are non-empty byte strings
  - Computes total_length as sum of all file lengths with overflow checking
- Updated parse function to detect multi-file format (files key present, no length key)
- Added comprehensive test suite covering:
  - Single file in multi-file format
  - Multiple files
  - Nested directory structures
  - Error cases: empty files array, empty path, missing length/path, non-dict entries, negative length, non-string path components, missing both length and files

All 154 tests pass, linting clean.
<!-- SECTION:NOTES:END -->
