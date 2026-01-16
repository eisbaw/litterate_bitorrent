---
id: task-0112
title: Implement PeerConnection struct
status: To Do
assignee: []
created_date: '2026-01-16 21:37'
labels:
  - phase-3
  - network
  - peer
dependencies:
  - task-0088
  - task-0106
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Encapsulate TCP stream, handshake result, and peer state into a single struct representing a live peer connection. This provides a clean interface for the orchestration layer to interact with peers without managing low-level details.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PeerConnection struct with: stream: TcpStream, peer_id: [u8; 20], state: PeerState, last_recv_time: Instant, last_send_time: Instant
- [ ] #2 PeerConnection::new(stream, peer_handshake, num_pieces) constructs from completed handshake
- [ ] #3 Stores peer_id from received handshake for identification
- [ ] #4 Initializes PeerState with correct number of pieces
- [ ] #5 Tracks timing for keep-alive and idle timeout
<!-- AC:END -->
