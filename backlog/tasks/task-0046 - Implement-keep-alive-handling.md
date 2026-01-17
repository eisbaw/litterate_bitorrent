---
id: task-0046
title: Implement keep-alive handling
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 22:34'
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
- [x] #1 Send keep-alive if no messages sent in 120 seconds
- [x] #2 Track last message time per peer
- [x] #3 Disconnect peer if no messages received in 180 seconds
- [x] #4 handle_keepalive() updates last-received timestamp (no-op otherwise)
- [x] #5 Unit test: keep-alive sent after idle period
- [x] #6 Unit test: idle peer is disconnected after timeout
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add PEER_IDLE_TIMEOUT_SECS constant (180 seconds) after KEEP_ALIVE_INTERVAL_SECS in the framed message writing section
2. Extend handle_keep_alives() function to check last_recv_time for each peer and disconnect idle peers by calling handle_peer_disconnected
3. Verify that handle_keepalive (Message::KeepAlive case) already calls mark_received() via the existing conn.mark_received() call (already confirmed in code at line 7678)
4. Add unit test for keep-alive sending after idle period in framed_message_writing_tests
5. Add unit test for idle peer disconnection in a new keep_alive_tests module
6. Run just lint and just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Two-pass approach: disconnect idle peers first, then send keep-alives. 120s send interval, 180s idle timeout.
<!-- SECTION:NOTES:END -->
