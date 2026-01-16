---
id: task-0095
title: Implement cancel message sending on piece completion
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
labels:
  - phase-5
  - strategy
  - cancel
dependencies:
  - task-0067
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When a piece is completed by receiving the final block from one peer, any pending requests for blocks of that piece to other peers must be cancelled. This avoids wasting bandwidth and allows those peers to serve other requests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cancel_piece_requests() takes piece_index and list of all peer queues
- [ ] #2 Returns list of (peer_id, CancelMessage) pairs for all pending requests
- [ ] #3 Removes cancelled requests from each peer's queue
- [ ] #4 Does not cancel from the peer that completed the piece (they already delivered)
- [ ] #5 Unit test: cancels requests from multiple peers correctly
- [ ] #6 Unit test: excludes the completing peer from cancellation
- [ ] #7 Unit test: handles case where no other peers had pending requests
<!-- AC:END -->
