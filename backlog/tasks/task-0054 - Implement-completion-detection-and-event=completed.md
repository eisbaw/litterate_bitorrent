---
id: task-0054
title: Implement completion detection and event=completed
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - completion
  - tracker
dependencies:
  - task-0031
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Detects when all pieces have been downloaded and verified. Sends event=completed to tracker to transition from leecher to seeder status (though we do not actually seed per non-goals).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Check after each piece verification if all pieces are complete
- [ ] #2 Send tracker announce with event=completed when download finishes
- [ ] #3 Log completion at INFO level with total time and average speed
- [ ] #4 Set internal state to prevent further requesting
- [ ] #5 Handle tracker failure on completed announce (log warning, not fatal)
- [ ] #6 Unit test: completion detected when last piece verifies
- [ ] #7 Integration test: completed event sent to tracker
<!-- AC:END -->
