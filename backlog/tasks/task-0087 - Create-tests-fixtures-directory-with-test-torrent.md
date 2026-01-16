---
id: task-0087
title: Create tests/fixtures directory with test torrent
status: To Do
assignee: []
created_date: '2026-01-16 21:32'
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
- [ ] #1 tests/ directory exists
- [ ] #2 tests/fixtures/ directory exists
- [ ] #3 A small .torrent file exists in tests/fixtures/
- [ ] #4 The torrent file is valid (parseable bencode)
- [ ] #5 The torrent is for public domain content
- [ ] #6 The torrent is small enough to download quickly in tests
<!-- AC:END -->
