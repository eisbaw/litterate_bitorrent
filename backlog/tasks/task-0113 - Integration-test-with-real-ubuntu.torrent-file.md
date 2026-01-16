---
id: task-0113
title: Integration test with real ubuntu.torrent file
status: To Do
assignee: []
created_date: '2026-01-16 21:37'
updated_date: '2026-01-16 21:38'
labels:
  - metainfo
  - integration
  - phase1
dependencies:
  - task-0114
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Verify complete Phase 1 implementation by parsing a real torrent file. This is the exit criteria validation: load tests/fixtures/ubuntu.torrent, print info hash, piece count, and total size. Confirms end-to-end correctness.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ubuntu.torrent file exists in tests/fixtures/
- [ ] #2 Successfully decode the bencode
- [ ] #3 Successfully parse Metainfo from bencoded dict
- [ ] #4 Print info hash in hex format
- [ ] #5 Print correct piece count (total_length / piece_length rounded up)
- [ ] #6 Print total size in bytes and human-readable format
- [ ] #7 All values match known correct values for the torrent
- [ ] #8 Test passes in CI
<!-- AC:END -->
