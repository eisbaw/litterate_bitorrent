---
id: task-0103
title: 'Integration test: announce to real tracker and print peers'
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:34'
updated_date: '2026-01-18 03:14'
labels:
  - tracker
  - integration-test
  - e2e
dependencies:
  - task-0097
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
End-to-end verification that Phase 2 works: load a real .torrent file, perform announce, print discovered peers. This is the exit criteria for Phase 2. Use a well-seeded public torrent (e.g., Ubuntu ISO) that is reliably available. The test proves the entire tracker flow works against real-world infrastructure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Test loads a real .torrent file (fixture in tests/fixtures/)
- [x] #2 Performs announce to actual tracker
- [x] #3 Prints list of peer IP:port pairs to stdout
- [x] #4 Handles case where tracker returns no peers (not an error, just empty list)
- [x] #5 Test can be run via just e2e or cargo test --ignored
- [x] #6 Documents any required network access in test comments
- [x] #7 Exit criteria: Given a .torrent, prints list of peer IP:port pairs from tracker
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create integration test file in tests/ directory
2. Write test that loads ubuntu.torrent from fixtures
3. Parse metainfo and create TrackerRequest
4. Call announce() function to contact tracker
5. Print peer list to stdout (or handle empty list gracefully)
6. Mark test with #[ignore] for network access
7. Update justfile e2e recipe to run the ignored test
8. Run just tangle && just lint && just test to verify
9. Document network requirements in test comments
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added tests/tracker_integration.rs with test_announce_to_real_tracker:
- Loads ubuntu.torrent from tests/fixtures/
- Announces to real Ubuntu tracker
- Prints peer IP:port pairs to stdout
- Handles empty peer list gracefully
- Marked #[ignore] for network requirement
- Run with just e2e or cargo test --ignored
<!-- SECTION:NOTES:END -->
