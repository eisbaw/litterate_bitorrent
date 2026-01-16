---
id: task-0027
title: Implement choke/unchoke message handler
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:30'
labels:
  - phase-6
  - message-handler
  - peer-protocol
dependencies:
  - task-0006
  - task-0015
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles incoming choke and unchoke messages from peers. Choke state controls whether we can request blocks from a peer. When unchoked, we should start requesting; when choked, we must stop requesting and mark pending requests for re-request from other peers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 handle_unchoke() sets peer_choking=false, triggers block requesting
- [ ] #2 handle_choke() sets peer_choking=true, cancels pending requests to this peer
- [ ] #3 Choked requests are returned to the piece manager for reassignment
- [ ] #4 Logs state transitions at INFO level per PRD
- [ ] #5 Unit test: unchoke enables requesting, choke disables it
- [ ] #6 Unit test: pending requests are properly cancelled on choke
<!-- AC:END -->
