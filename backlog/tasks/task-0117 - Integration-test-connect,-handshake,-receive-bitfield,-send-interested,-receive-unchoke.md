---
id: task-0117
title: >-
  Integration test: connect, handshake, receive bitfield, send interested,
  receive unchoke
status: To Do
assignee: []
created_date: '2026-01-16 21:40'
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
- [ ] #1 Test connects to a real BitTorrent peer (using a well-seeded torrent)
- [ ] #2 Successfully completes handshake (validates info_hash match)
- [ ] #3 Receives and parses bitfield message from peer
- [ ] #4 Sends Interested message to peer
- [ ] #5 Receives Unchoke message from peer (may take a few seconds)
- [ ] #6 Test completes within 30 seconds timeout
- [ ] #7 Test can be run with 'just test-peer-handshake' or similar
<!-- AC:END -->
