---
id: task-0096
title: Implement PeerState message handlers
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
labels:
  - phase-3
  - protocol
  - peer
  - state
dependencies:
  - task-0088
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
PeerState must update correctly when receiving protocol messages. Each message type changes specific state: choke/unchoke updates peer_choking, interested/not_interested updates peer_interested, have/bitfield updates the bitfield, etc. This is the state machine core.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PeerState::handle_choke() sets peer_choking=true and clears pending_requests
- [ ] #2 PeerState::handle_unchoke() sets peer_choking=false
- [ ] #3 PeerState::handle_interested() sets peer_interested=true
- [ ] #4 PeerState::handle_not_interested() sets peer_interested=false
- [ ] #5 PeerState::handle_have(index: u32) sets bit in bitfield, validates index
- [ ] #6 PeerState::handle_bitfield(bits: &[u8]) -> Result<(), Error> sets bitfield, validates length matches num_pieces
- [ ] #7 Unit tests verify state transitions for each handler
<!-- AC:END -->
