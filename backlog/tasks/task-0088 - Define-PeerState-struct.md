---
id: task-0088
title: Define PeerState struct
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:32'
updated_date: '2026-01-17 17:56'
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
- [x] #1 PeerState struct with: am_choking: bool (default true), am_interested: bool (default false), peer_choking: bool (default true), peer_interested: bool (default false)
- [x] #2 PeerState includes bitfield: Vec<u8> for tracking which pieces peer has
- [x] #3 PeerState includes pending_requests: VecDeque<BlockRequest> for in-flight requests
- [x] #4 PeerState::new(num_pieces: u32) initializes with defaults and empty bitfield of correct size
- [x] #5 PeerState::can_request() returns true only when !peer_choking && am_interested
- [x] #6 PeerState::has_piece(index: u32) -> bool checks bitfield
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read the existing 07-client.nw to find where PeerState placeholder exists
2. Replace placeholder PeerState with full struct: am_choking, am_interested, peer_choking, peer_interested (bools), bitfield (Vec<u8>), pending_requests (VecDeque<BlockRequest>)
3. Implement PeerState::new(num_pieces: u32) with correct defaults
4. Implement PeerState::can_request() returning \!peer_choking && am_interested
5. Implement PeerState::has_piece(index: u32) -> bool checking bitfield
6. Write literate prose explaining the 4-bit state machine
7. Add tests for all methods
8. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented PeerState struct in nw/07-client.nw (client module).

Key changes:
- Replaced placeholder PeerState with full implementation
- Fields: am_choking, am_interested, peer_choking, peer_interested (bools), bitfield (Vec<u8>), pending_requests (VecDeque<BlockRequest>)
- PeerState::new(num_pieces) initializes with protocol defaults: both sides choking, neither interested
- PeerState::can_request() returns true only when \!peer_choking && am_interested
- PeerState::has_piece(index) checks bitfield using MSB-first bit ordering
- Added helper methods: set_piece(), piece_count(), is_seeder()
- Implemented Default trait for testing convenience
- Added 14 unit tests covering all acceptance criteria

The bitfield uses BitTorrent convention (MSB-first): piece 0 is bit 7 of byte 0.
<!-- SECTION:NOTES:END -->
