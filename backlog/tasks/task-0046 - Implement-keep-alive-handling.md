---
id: task-0046
title: Implement keep-alive handling
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - keep-alive
  - peer-protocol
dependencies:
  - task-0015
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implements keep-alive message sending and receiving. Keep-alives maintain TCP connections during idle periods and detect dead peers. PRD specifies 120-second send interval and 180-second idle disconnect.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Send keep-alive if no messages sent in 120 seconds
- [ ] #2 Track last message time per peer
- [ ] #3 Disconnect peer if no messages received in 180 seconds
- [ ] #4 handle_keepalive() updates last-received timestamp (no-op otherwise)
- [ ] #5 Unit test: keep-alive sent after idle period
- [ ] #6 Unit test: idle peer is disconnected after timeout
<!-- AC:END -->
