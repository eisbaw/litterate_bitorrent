---
id: task-0076
title: Implement request pipelining logic
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
labels:
  - phase-5
  - strategy
  - pipelining
dependencies:
  - task-0056
  - task-0067
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Request pipelining sends multiple block requests before receiving responses, maximizing throughput by keeping the network pipe full. The strategy must fill each peer's request queue up to capacity with blocks from selected pieces, respecting piece selection order.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 fill_request_queue() generates requests to fill peer queue to capacity
- [ ] #2 Prioritizes completing in-progress pieces before starting new ones
- [ ] #3 Uses rarest-first for selecting new pieces to start
- [ ] #4 Returns list of BlockRequest to send to peer
- [ ] #5 Respects peer's bitfield (only request pieces they have)
- [ ] #6 Unit test: fills queue to exactly max_size when sufficient blocks available
- [ ] #7 Unit test: prefers blocks from in-progress pieces over starting new pieces
- [ ] #8 Unit test: returns empty when peer has no pieces we need
<!-- AC:END -->
