---
id: task-0103
title: 'Integration test: announce to real tracker and print peers'
status: To Do
assignee: []
created_date: '2026-01-16 21:34'
labels:
  - tracker
  - integration-test
  - e2e
dependencies:
  - task-0097
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
End-to-end verification that Phase 2 works: load a real .torrent file, perform announce, print discovered peers. This is the exit criteria for Phase 2. Use a well-seeded public torrent (e.g., Ubuntu ISO) that is reliably available. The test proves the entire tracker flow works against real-world infrastructure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Test loads a real .torrent file (fixture in tests/fixtures/)
- [ ] #2 Performs announce to actual tracker
- [ ] #3 Prints list of peer IP:port pairs to stdout
- [ ] #4 Handles case where tracker returns no peers (not an error, just empty list)
- [ ] #5 Test can be run via just e2e or cargo test --ignored
- [ ] #6 Documents any required network access in test comments
- [ ] #7 Exit criteria: Given a .torrent, prints list of peer IP:port pairs from tracker
<!-- AC:END -->
