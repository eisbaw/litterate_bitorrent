---
id: task-0006
title: Define Client struct holding all state
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - phase-6
  - orchestration
  - foundation
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Client struct is the central orchestration point that owns all torrent state: metainfo, peer connections, piece manager, download strategy, and tracker state. This is the foundation for the event loop and must be designed before any orchestration logic.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Client struct defined with fields for: metainfo, info_hash, peer_id, piece_manager, strategy, active_peers map, tracker_state
- [ ] #2 Client implements new() constructor that initializes all state from Metainfo
- [ ] #3 Client owns tokio runtime handles for spawned peer connections
- [ ] #4 Unit test verifies Client can be constructed from valid Metainfo
<!-- AC:END -->
