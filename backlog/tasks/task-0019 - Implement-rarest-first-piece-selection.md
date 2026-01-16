---
id: task-0019
title: Implement rarest-first piece selection
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - phase-5
  - strategy
  - selection
dependencies:
  - task-0012
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Core algorithm for BitTorrent piece selection. Rarest-first prioritizes pieces with lowest availability to improve swarm health and download speed. Must filter out pieces we already have or are in progress, then select from the remaining based on availability count.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 select_rarest() returns piece indices sorted by availability (ascending)
- [ ] #2 Excludes pieces already verified (Verified state)
- [ ] #3 Excludes pieces currently in progress (InProgress state)
- [ ] #4 Returns empty vec if no pieces are available from the given peer
- [ ] #5 Unit test: with availabilities [3, 1, 2], returns order [1, 2, 0]
- [ ] #6 Unit test: filters out already-completed pieces correctly
- [ ] #7 Unit test: returns empty when peer has no pieces we need
<!-- AC:END -->
