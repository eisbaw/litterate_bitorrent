---
id: task-0050
title: Implement re-announce timer
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - tracker
  - networking
dependencies:
  - task-0006
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implements periodic tracker re-announces. The tracker response includes an interval (typically 30 minutes). We must re-announce to get fresh peers and update our statistics (uploaded/downloaded/left).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Parse and store interval from tracker response
- [ ] #2 Schedule re-announce after interval elapses
- [ ] #3 Re-announce includes updated downloaded/uploaded/left statistics
- [ ] #4 Handle tracker failure gracefully (retry with backoff, continue with existing peers)
- [ ] #5 Merge new peers into available pool without duplicates
- [ ] #6 Logs re-announce at INFO level, failures at WARN
- [ ] #7 Unit test: re-announce scheduled correctly based on interval
- [ ] #8 Integration test: handles tracker temporary unavailability
<!-- AC:END -->
