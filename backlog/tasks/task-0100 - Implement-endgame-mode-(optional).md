---
id: task-0100
title: Implement endgame mode (optional)
status: To Do
assignee: []
created_date: '2026-01-16 21:34'
labels:
  - phase-5
  - strategy
  - endgame
  - optional
dependencies:
  - task-0095
  - task-0089
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Endgame mode activates when few pieces remain. In this mode, blocks are requested from multiple peers simultaneously to avoid stalling on slow peers. When a block arrives, cancel messages are sent to all other peers. This is optional per PRD but improves end-of-download performance.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 is_endgame_mode() returns true when remaining pieces below threshold (e.g., 5 or fewer pieces)
- [ ] #2 In endgame mode, same block can be requested from multiple peers
- [ ] #3 Track duplicate requests across peers
- [ ] #4 On block receipt, cancel duplicates from other peers
- [ ] #5 Threshold is configurable
- [ ] #6 Unit test: endgame mode activates at correct piece count
- [ ] #7 Unit test: duplicate requests are tracked correctly
- [ ] #8 Unit test: cancellations sent on receipt during endgame
<!-- AC:END -->
