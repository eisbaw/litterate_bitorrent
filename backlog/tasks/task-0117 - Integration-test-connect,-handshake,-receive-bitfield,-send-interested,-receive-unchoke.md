---
id: task-0117
title: >-
  Integration test: connect, handshake, receive bitfield, send interested,
  receive unchoke
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:40'
updated_date: '2026-01-18 03:30'
labels:
  - phase-3
  - integration-test
  - exit-criteria
dependencies:
  - task-0106
  - task-0109
  - task-0111
  - task-0096
  - task-0112
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
This is the Phase 3 exit criteria test. Verifies the complete peer connection flow works end-to-end against a real peer. This test proves handshake, message framing, and state machine work correctly together.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Test connects to a real BitTorrent peer (using a well-seeded torrent)
- [x] #2 Successfully completes handshake (validates info_hash match)
- [x] #3 Receives and parses bitfield message from peer
- [x] #4 Sends Interested message to peer
- [x] #5 Receives Unchoke message from peer (may take a few seconds)
- [x] #6 Test completes within 30 seconds timeout
- [x] #7 Test can be run with 'just test-peer-handshake' or similar
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create tests/peer_integration.rs with the peer connection integration test
2. Reuse existing infrastructure: load_ubuntu_torrent(), create_announce_request() from tracker_integration.rs
3. Test flow:
   - Load ubuntu.torrent, announce to tracker to get peers
   - Try connecting to each peer until one responds (10 second connect timeout)
   - Perform handshake and validate info_hash match
   - Wait for bitfield message
   - Send Interested message
   - Wait for Unchoke message (with 30 second overall timeout)
4. Add just test-peer-handshake recipe to justfile
5. Run just tangle && just lint && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added tests/peer_integration.rs with test_peer_connection_flow:
- Connects to real BitTorrent peers from Ubuntu tracker
- Completes handshake with info_hash validation
- Receives bitfield/Have messages
- Sends Interested message
- Receives Unchoke (handles optimistic unchoking)
- 30-second timeout, marked #[ignore]
- Run with just test-peer-handshake
<!-- SECTION:NOTES:END -->
