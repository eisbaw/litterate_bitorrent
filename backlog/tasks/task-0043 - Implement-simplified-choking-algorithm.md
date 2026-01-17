---
id: task-0043
title: Implement simplified choking algorithm
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 23:14'
labels:
  - phase-6
  - choking
  - peer-protocol
dependencies:
  - task-0015
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implements our choking decisions toward peers. Per PRD, use simplified algorithm: unchoke all interested peers. We are a leecher, so we unchoke everyone to encourage reciprocity. This is simpler than full tit-for-tat but sufficient for downloading.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 unchoke_interested_peers() sends unchoke to all interested peers
- [x] #2 Runs periodically (every 10 seconds) to catch new interested peers
- [x] #3 Sends choke when peer becomes uninterested (optional optimization)
- [x] #4 Logs choke/unchoke decisions at DEBUG level
- [x] #5 Unit test: interested peers receive unchoke message
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add CHOKE_INTERVAL_SECS constant (10 seconds)
2. Add ChokingReevaluation variant to EventLoopEvent enum
3. Create unchoke_interested_peers() function that iterates peers and sends Unchoke to interested-but-choked peers
4. Add choking interval timer to run_event_loop()
5. Add ChokingReevaluation case to handle_event() dispatcher
6. Add unit test for unchoke_interested_peers()
7. Run lint and tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented simplified choking algorithm for leecher-only client:

- Added CHOKE_INTERVAL_SECS constant (10 seconds) for periodic reevaluation
- Added ChokingReevaluation event to EventLoopEvent enum
- Created handle_choking_reevaluation() function that:
  - Sends Unchoke to interested peers we are currently choking
  - Optionally sends Choke to uninterested peers we are not choking (resource optimization)
  - Logs decisions at DEBUG level via tracing
- Integrated choking timer into run_event_loop() with other periodic timers
- Added handler dispatch in handle_event() match arm

Tests added:
- choke_interval_is_reasonable: validates 10s is between 5-30s bounds
- choking_reevaluation_event_debug: verifies event Debug impl
- choking_decision_interested_and_choking_should_unchoke: AC #5 unit test
- choking_decision_not_interested_and_unchoked_should_choke: tests optional choke optimization

All changes in nw/07-client.nw (literate noweb file).
<!-- SECTION:NOTES:END -->
