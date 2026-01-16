---
id: task-0089
title: Implement request timeout handling and re-request
status: To Do
assignee: []
created_date: '2026-01-16 21:32'
labels:
  - phase-5
  - strategy
  - timeout
dependencies:
  - task-0083
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When a request times out, it must be cancelled with the original peer and re-requested from another peer that has the piece. This involves: removing from the original peer's queue, sending cancel message, finding alternative peer, and queuing the request there.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 handle_timeout() takes timed-out BlockRequest and peer list
- [ ] #2 Removes request from original peer's queue
- [ ] #3 Returns CancelMessage to send to original peer
- [ ] #4 Selects alternative peer that has the piece and is not choking us
- [ ] #5 Returns new BlockRequest for alternative peer, or None if no alternative available
- [ ] #6 Marks block as not-in-flight so it can be re-requested
- [ ] #7 Unit test: selects alternative peer that has the piece
- [ ] #8 Unit test: returns None when no alternative peer available
- [ ] #9 Unit test: generates correct cancel message format
<!-- AC:END -->
