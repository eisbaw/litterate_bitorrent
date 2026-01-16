---
id: task-0068
title: Handle tracker unreachable during download
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
updated_date: '2026-01-16 21:32'
labels:
  - phase-6
  - recovery
  - tracker
dependencies:
  - task-0050
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles the case where tracker becomes unreachable during an active download. The download should continue with existing peers. This is part of graceful degradation per PRD recovery strategy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Re-announce failure does not stop the download
- [ ] #2 Retry tracker with exponential backoff (30s, 60s, 120s, cap at 15 min)
- [ ] #3 Continue downloading with existing peer pool
- [ ] #4 Log tracker failure at WARN level
- [ ] #5 Resume normal re-announce interval after successful retry
- [ ] #6 Unit test: tracker failure triggers backoff
- [ ] #7 Integration test: download completes despite tracker outage
<!-- AC:END -->
