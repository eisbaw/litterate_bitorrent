---
id: task-0063
title: Implement peer disconnect recovery
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - recovery
  - error-handling
dependencies:
  - task-0015
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles peer disconnections during download. When peers disconnect (network issues, choking permanently, or reaching upload limits), we must clean up their state and attempt to replace them with fresh peers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Detect peer disconnect via TCP connection close or read error
- [ ] #2 Clean up peer state: remove from active peers, cancel pending requests
- [ ] #3 Return cancelled requests to piece manager for reassignment
- [ ] #4 Update strategy availability (peer no longer has those pieces)
- [ ] #5 Attempt to connect replacement peer from pool
- [ ] #6 Handle all-peers-disconnected case: wait for re-announce to get new peers
- [ ] #7 Log disconnect at INFO level with reason if available
- [ ] #8 Unit test: disconnect cleanup is complete
- [ ] #9 Integration test: download continues after peer disconnect
<!-- AC:END -->
