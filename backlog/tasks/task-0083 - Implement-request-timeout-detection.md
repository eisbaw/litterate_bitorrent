---
id: task-0083
title: Implement request timeout detection
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
labels:
  - phase-5
  - strategy
  - timeout
dependencies:
  - task-0067
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Block requests that exceed timeout (30s per PRD) must be detected so they can be cancelled and re-requested from another peer. Timeout detection scans pending requests and identifies those past the deadline. This is separate from the action taken (cancellation).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 check_timeouts() returns list of timed-out BlockRequests
- [ ] #2 Timeout threshold is configurable (default 30 seconds)
- [ ] #3 Uses requested_at timestamp from BlockRequest for comparison
- [ ] #4 Timed-out requests remain in queue until explicitly removed (caller's responsibility)
- [ ] #5 Unit test: request at T=0, check at T=31s returns it as timed out
- [ ] #6 Unit test: request at T=0, check at T=29s does not return it
- [ ] #7 Unit test: multiple requests, only those past timeout are returned
<!-- AC:END -->
