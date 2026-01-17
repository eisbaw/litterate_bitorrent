---
id: task-0112
title: Implement PeerConnection struct
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:37'
updated_date: '2026-01-17 19:09'
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
- [x] #1 PeerConnection struct with: stream: TcpStream, peer_id: [u8; 20], state: PeerState, last_recv_time: Instant, last_send_time: Instant
- [x] #2 PeerConnection::new(stream, peer_handshake, num_pieces) constructs from completed handshake
- [x] #3 Stores peer_id from received handshake for identification
- [x] #4 Initializes PeerState with correct number of pieces
- [x] #5 Tracks timing for keep-alive and idle timeout
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update PeerConnection struct to add stream, peer_id, last_recv_time, last_send_time fields
2. Add PeerConnection::new constructor with stream, addr, peer_handshake, num_pieces params
3. Add mark_received() and mark_sent() helper methods
4. Add register_connection method to PeerConnectionManager
5. Update connect_to_peer method (deprecate or remove)
6. Add tests for new PeerConnection functionality
7. Run just check and cargo test peer_connection to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated PeerConnection struct in nw/07-client.nw to encapsulate live peer connection.

Struct fields:
- stream: TcpStream - TCP connection
- addr: SocketAddr - peer address
- peer_id: [u8; 20] - from handshake
- state: PeerState - protocol state machine
- last_recv_time / last_send_time: Instant - for timeout/keep-alive

Methods:
- new(stream, addr, peer_handshake, num_pieces) - constructor
- mark_received() / mark_sent() - update timing
- Custom Debug impl (omits stream, shows peer_id first 8 bytes)

Manager API updated:
- Replaced connect_to_peer placeholder with register_connection(stream, addr, handshake)

Tests: 5 new tests for PeerConnection + updated manager tests

All 672 tests pass.
<!-- SECTION:NOTES:END -->
