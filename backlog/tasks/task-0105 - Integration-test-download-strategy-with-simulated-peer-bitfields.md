---
id: task-0105
title: 'Integration test: download strategy with simulated peer bitfields'
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:35'
updated_date: '2026-01-18 02:33'
labels:
  - phase-5
  - strategy
  - integration-test
dependencies:
  - task-0076
  - task-0089
  - task-0095
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Verify the complete download strategy works end-to-end with simulated peers. This test validates rarest-first selection, request pipelining, and timeout handling work together correctly. Uses deterministic peer bitfields to verify expected behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Test fixture creates simulated swarm with 5+ peers having varying bitfields
- [x] #2 Test verifies rarest piece is selected first
- [x] #3 Test verifies request queue fills to capacity
- [x] #4 Test verifies block completion removes request from queue
- [x] #5 Test verifies piece completion triggers cancel messages to other peers
- [x] #6 Test verifies timeout triggers re-request from alternative peer
- [x] #7 Test passes with deterministic RNG for reproducibility
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read existing test patterns and understand helper structures (PieceManager, RequestScheduler, PieceAvailability)
2. Design SimulatedPeer struct to model peers with bitfields and choke state
3. Design SimulatedSwarm struct to manage multiple peers
4. Write integration test: setup_simulated_swarm_creates_five_peers
5. Write integration test: verify_rarest_piece_selected_first
6. Write integration test: verify_request_queue_fills_to_capacity
7. Write integration test: verify_block_completion_removes_from_queue
8. Write integration test: verify_piece_completion_returns_cancel_info
9. Write integration test: verify_timeout_triggers_rerequest_from_alternate_peer
10. Add prose explaining the tests
11. Include test chunk in file assembly
12. Run just tangle, lint, test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added 8 integration tests with SimulatedPeer/SimulatedSwarm helpers.
Tests verify rarest-first selection, request pipelining, cancel message generation,
timeout handling, and deterministic behavior with seeded RNG.
All tests pass with 878 total tests.
<!-- SECTION:NOTES:END -->
