---
id: task-0117
title: >-
  Integration test: connect, handshake, receive bitfield, send interested,
  receive unchoke
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:40'
updated_date: '2026-01-18 03:26'
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
Implemented peer connection integration test in tests/peer_integration.rs.

## Changes
- Created new test file tests/peer_integration.rs with:
  - test_peer_connection_flow: Full integration test that connects to real BitTorrent peers
  - test_peer_integration_setup: Non-network test to verify fixture loading and module setup

## Test Flow
1. Loads ubuntu.torrent fixture
2. Announces to Ubuntu tracker to get peer list
3. Connects to peers with 10-second timeout
4. Performs handshake and validates info_hash match
5. Waits for bitfield or Have messages
6. Sends Interested message
7. Waits for Unchoke message (or detects early unchoke)

## Edge Cases Handled
- Early unchoke: Some peers send Unchoke before we send Interested (optimistic unchoking)
- No bitfield: Some peers send only Have messages instead of a bitfield
- Tracker returns no peers: Fails with clear message
- All peers unreachable: Tries up to 10 peers before failing
- Timeout: Overall 30-second test timeout with 15-second unchoke wait

## Justfile
Added `just test-peer-handshake` recipe to run only the peer integration test.

## Testing
- All unit tests pass (895 tests)
- E2E tests pass (successfully connects to Ubuntu peer and receives unchoke)
<!-- SECTION:NOTES:END -->
