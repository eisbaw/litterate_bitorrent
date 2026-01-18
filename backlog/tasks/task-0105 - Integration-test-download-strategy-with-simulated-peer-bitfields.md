---
id: task-0105
title: 'Integration test: download strategy with simulated peer bitfields'
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:35'
updated_date: '2026-01-18 02:30'
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
Implemented integration tests for the download strategy in nw/05-pieces.nw.

Added test helper structures:
- SimulatedPeer: Models a connected peer with peer_id, bitfield, and choke state
- SimulatedSwarm: Manages multiple peers and tracks piece availability

Added 8 integration tests:
1. integration_simulated_swarm_creates_peers_with_varying_bitfields - Validates swarm setup with 5+ peers
2. integration_rarest_piece_selected_first - Verifies rarest-first piece selection
3. integration_request_queue_fills_to_capacity - Tests pipeline limit enforcement
4. integration_block_completion_removes_from_queue - Tests block completion tracking
5. integration_piece_completion_identifies_peers_for_cancel - Tests cancel message identification
6. integration_timeout_triggers_rerequest_from_alternate_peer - Tests timeout and re-request workflow
7. integration_strategy_deterministic_with_seeded_rng - Verifies reproducibility with seeded RNG
8. integration_full_download_workflow_with_multiple_peers - Comprehensive end-to-end test

All tests pass with just tangle, lint, and test commands.
<!-- SECTION:NOTES:END -->
