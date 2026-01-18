---
id: task-0121
title: Implement connection health monitoring
status: To Do
assignee: []
created_date: '2026-01-18 09:47'
labels:
  - reliability
  - performance
  - client
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add detection and removal of connections that haven't transmitted useful data for 180+ seconds. Currently dead connections stay open and waste resources. Track peer quality metrics to prioritize healthy connections.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Track last useful activity timestamp per peer (block received, bitfield update)
- [ ] #2 Add periodic health check in event loop (every 30 seconds)
- [ ] #3 Disconnect peers with no useful activity for 180+ seconds
- [ ] #4 Log peer disconnection reason at INFO level
- [ ] #5 Replace disconnected unhealthy peers with fresh ones from pool
- [ ] #6 Unit test: stale peer detection triggers disconnect
<!-- AC:END -->
