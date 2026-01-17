---
id: task-0063
title: Implement peer disconnect recovery
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 22:25'
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
- [x] #1 Detect peer disconnect via TCP connection close or read error
- [x] #2 Clean up peer state: remove from active peers, cancel pending requests
- [x] #3 Return cancelled requests to piece manager for reassignment
- [x] #4 Update strategy availability (peer no longer has those pieces)
- [x] #5 Attempt to connect replacement peer from pool
- [x] #6 Handle all-peers-disconnected case: wait for re-announce to get new peers
- [x] #7 Log disconnect at INFO level with reason if available
- [x] #8 Unit test: disconnect cleanup is complete
- [x] #9 Integration test: download continues after peer disconnect
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Enhance handle_peer_disconnected to drain pending requests before removing queue
2. Add request reassignment logic - find alternative peers for pending blocks
3. Add ConnectMorePeers event for signaling need for new connections
4. Handle all-peers-disconnected by checking if active_count == 0
5. Add TriggerReAnnounce action for requesting fresh peers when pool is empty
6. Write unit test for disconnect cleanup completeness
7. Write integration test for download continuation
8. Run just lint and just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Full disconnect recovery with request reassignment. 13 unit tests added. Alternative peer selection is simple (first match) - can be enhanced later for load balancing.
<!-- SECTION:NOTES:END -->
