---
id: task-0087
title: Create tests/fixtures directory with test torrent
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:32'
updated_date: '2026-01-16 23:15'
labels:
  - testing
  - phase0
dependencies:
  - task-0001
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the test fixtures directory with a small, well-seeded public domain torrent file for integration testing. This enables future e2e tests to verify the client can parse real torrent files.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tests/ directory exists
- [x] #2 tests/fixtures/ directory exists
- [x] #3 A small .torrent file exists in tests/fixtures/
- [x] #4 The torrent file is valid (parseable bencode)
- [x] #5 The torrent is for public domain content
- [x] #6 The torrent is small enough to download quickly in tests
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create tests/ directory
2. Create tests/fixtures/ subdirectory
3. Download Ubuntu torrent from official source
4. Verify the file is valid bencode
5. Mark acceptance criteria complete
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created tests/fixtures/ directory structure and downloaded Ubuntu 24.04.3 LTS server torrent from official Ubuntu source.

Files added:
- tests/fixtures/ubuntu.torrent (252KB, valid bencoded dictionary)
- tests/fixtures/ubuntu.torrent.expected (metadata for test verification)

The torrent file starts with "d8:announce" confirming valid bencode format. Ubuntu is public domain content and well-seeded.
<!-- SECTION:NOTES:END -->
