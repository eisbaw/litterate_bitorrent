---
id: task-0123
title: Implement per-block request timeout with re-request
status: To Do
assignee: []
created_date: '2026-01-18 09:48'
labels:
  - performance
  - reliability
  - client
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Enforce per-block request timeouts (30s as per PRD). When a block request times out, automatically re-request from another peer that has the piece. This prevents stalled downloads when a peer becomes unresponsive mid-piece.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Track request timestamp for each pending block request
- [ ] #2 Add timeout check in event loop (every 5 seconds)
- [ ] #3 On timeout, cancel request and mark block as available for re-request
- [ ] #4 Select alternative peer for timed-out block (prefer peers with same piece)
- [ ] #5 Log timeout events at DEBUG level with peer and block info
- [ ] #6 Unit test: timeout triggers re-request
- [ ] #7 Configurable timeout via SchedulerConfig (default 30s)
<!-- AC:END -->
