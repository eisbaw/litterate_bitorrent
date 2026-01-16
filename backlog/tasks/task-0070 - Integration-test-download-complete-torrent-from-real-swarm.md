---
id: task-0070
title: 'Integration test: download complete torrent from real swarm'
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
updated_date: '2026-01-16 21:32'
labels:
  - phase-6
  - integration-test
  - e2e
dependencies:
  - task-0060
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
End-to-end integration test that downloads a real torrent from a public swarm. This is the exit criterion for Phase 6 - proving the client actually works with real BitTorrent infrastructure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Test downloads a small, well-seeded torrent (e.g., test torrent or small Linux ISO)
- [ ] #2 Verifies downloaded file matches expected SHA1 hash
- [ ] #3 Test completes within reasonable timeout (5 minutes for small file)
- [ ] #4 Test runs in CI environment (just e2e)
- [ ] #5 Handles flaky network conditions (retry on transient failures)
- [ ] #6 Documents the test torrent used and its expected hash
- [ ] #7 Cleans up downloaded files after test
<!-- AC:END -->
