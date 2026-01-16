---
id: task-0088
title: Define PeerState struct
status: To Do
assignee: []
created_date: '2026-01-16 21:32'
labels:
  - phase-3
  - protocol
  - peer
  - state
dependencies:
  - task-0052
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Track the 4-bit choking/interest state machine for each peer connection, plus their bitfield and pending requests. The state determines whether data transfer can occur: we can only request pieces when peer is not choking us AND we are interested. This struct is the core of peer connection management.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PeerState struct with: am_choking: bool (default true), am_interested: bool (default false), peer_choking: bool (default true), peer_interested: bool (default false)
- [ ] #2 PeerState includes bitfield: Vec<u8> for tracking which pieces peer has
- [ ] #3 PeerState includes pending_requests: VecDeque<BlockRequest> for in-flight requests
- [ ] #4 PeerState::new(num_pieces: u32) initializes with defaults and empty bitfield of correct size
- [ ] #5 PeerState::can_request() returns true only when !peer_choking && am_interested
- [ ] #6 PeerState::has_piece(index: u32) -> bool checks bitfield
<!-- AC:END -->
