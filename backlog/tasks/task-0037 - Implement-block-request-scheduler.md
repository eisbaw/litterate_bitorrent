---
id: task-0037
title: Implement block request scheduler
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - request-scheduling
  - data-path
dependencies:
  - task-0027
  - task-0023
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Schedules block requests to unchoked peers. Implements request pipelining (10-16 concurrent requests per peer as per PRD) to maximize throughput. Requests pieces selected by the strategy module.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 request_blocks() sends requests to unchoked peers up to pipeline limit
- [ ] #2 Respects per-peer pipeline limit (configurable, default 10)
- [ ] #3 Queries strategy module for next piece to request
- [ ] #4 Tracks pending requests with timestamps for timeout detection
- [ ] #5 Implements 30-second request timeout per PRD
- [ ] #6 Timed-out requests are cancelled and reassigned to other peers
- [ ] #7 Does not request blocks already pending from another peer (except endgame)
- [ ] #8 Unit test: pipeline limit is respected
- [ ] #9 Unit test: timeout triggers request reassignment
<!-- AC:END -->
