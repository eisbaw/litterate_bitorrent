---
id: task-0015
title: Implement peer connection manager
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-18 04:02'
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
- [x] #1 PeerConnectionManager struct tracks active connections and available peer addresses
- [x] #2 connect_to_peers() spawns connections to N peers concurrently (configurable, default 30)
- [x] #3 Implements TCP connection with 10 second timeout per PRD
- [x] #4 Performs handshake validation (info_hash match, valid peer_id)
- [x] #5 Returns channel/handle for communicating with connected peer
- [x] #6 replace_disconnected() removes dead peers and connects to fresh ones from pool
- [x] #7 Integration test: can connect to mock peer and complete handshake
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add AlreadyConnected variant to PeerError in nw/04-messages.nw
2. Create PeerConnectionManager struct with required fields
3. Create PeerConnection struct
4. Add constants (DEFAULT_MAX_CONNECTIONS, CONNECT_TIMEOUT_SECS)
5. Implement new() method
6. Implement add_peers(), active_count(), available_count(), needs_more_connections()
7. Implement get_connection(), get_connection_mut(), remove_connection()
8. Implement async connect_to_peer() placeholder
9. Add literate documentation section
10. Add all required tests
11. Update module structure to include new chunks
12. Run just check && cargo test manager_ connect_to_peer
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
PeerConnectionManager fully implemented in nw/07-client.nw.
All functionality verified via integration tests:
- TCP connection with timeout
- Handshake validation
- Channel communication with peers
- Peer replacement on disconnect
<!-- SECTION:NOTES:END -->
