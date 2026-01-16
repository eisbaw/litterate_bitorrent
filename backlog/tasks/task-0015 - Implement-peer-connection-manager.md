---
id: task-0015
title: Implement peer connection manager
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 21:30'
labels:
  - phase-6
  - orchestration
  - networking
dependencies:
  - task-0006
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Manages the pool of active peer connections. Responsible for connecting to N peers concurrently (from tracker peer list), tracking connection state, and replacing disconnected peers with fresh ones from the pool. This decouples connection management from the main event loop.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PeerConnectionManager struct tracks active connections and available peer addresses
- [ ] #2 connect_to_peers() spawns connections to N peers concurrently (configurable, default 30)
- [ ] #3 Implements TCP connection with 10 second timeout per PRD
- [ ] #4 Performs handshake validation (info_hash match, valid peer_id)
- [ ] #5 Returns channel/handle for communicating with connected peer
- [ ] #6 replace_disconnected() removes dead peers and connects to fresh ones from pool
- [ ] #7 Integration test: can connect to mock peer and complete handshake
<!-- AC:END -->
