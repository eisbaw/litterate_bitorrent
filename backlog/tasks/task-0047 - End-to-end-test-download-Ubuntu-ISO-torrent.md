---
id: task-0047
title: 'End-to-end test: download Ubuntu ISO torrent'
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 21:29'
labels:
  - testing
  - e2e
  - phase7
dependencies:
  - task-0025
  - task-0028
  - task-0033
  - task-0021
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The ultimate validation: download a real torrent from start to finish. This proves the entire system works together - bencode parsing, tracker communication, peer protocol, piece management, and disk I/O. Ubuntu ISO is ideal: well-seeded, legal, large enough to test real behavior, small enough to be practical.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Download starts with: literate-bittorrent ubuntu.torrent -o ./downloads
- [ ] #2 Progress display shows pieces completing over time
- [ ] #3 Download completes successfully (all pieces verified)
- [ ] #4 Downloaded file matches expected SHA256 hash
- [ ] #5 Ctrl+C during download triggers graceful shutdown
- [ ] #6 Resume after interruption works (does not re-download verified pieces)
- [ ] #7 Test is added to justfile as 'just e2e' recipe
- [ ] #8 Test can run in CI (or is marked as manual with instructions)
<!-- AC:END -->
