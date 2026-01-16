---
id: task-0108
title: Implement multi-file torrent metainfo parsing
status: To Do
assignee: []
created_date: '2026-01-16 21:35'
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
- [ ] #1 Detects multi-file format (files array present, no length field)
- [ ] #2 Parses files array correctly
- [ ] #3 Extracts length from each file entry
- [ ] #4 Joins path components with OS path separator
- [ ] #5 Handles nested directories in path
- [ ] #6 Computes total_length as sum of all file lengths
- [ ] #7 Returns error for malformed files array
- [ ] #8 Unit test with hand-crafted multi-file torrent dict
<!-- AC:END -->
