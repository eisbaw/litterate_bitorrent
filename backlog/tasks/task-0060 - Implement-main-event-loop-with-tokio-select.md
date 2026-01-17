---
id: task-0060
title: Implement main event loop with tokio select
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 21:49'
labels:
  - phase-6
  - event-loop
  - orchestration
dependencies:
  - task-0023
  - task-0027
  - task-0031
  - task-0037
  - task-0043
  - task-0046
  - task-0050
  - task-0054
  - task-0058
  - task-0063
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The central event loop that multiplexes all async operations using tokio::select!. This is the heart of the client, orchestrating peer messages, timers, and shutdown signals. Must be implemented after individual handlers are ready.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Event loop uses tokio::select! to await multiple futures concurrently
- [x] #2 Handles: peer messages, request timeouts, re-announce timer, keep-alive timer, shutdown signal
- [x] #3 Routes incoming messages to appropriate handlers
- [x] #4 Triggers block requests after unchoke or piece completion
- [x] #5 Replaces disconnected peers from pool
- [x] #6 Exits cleanly on completion or shutdown signal
- [x] #7 Logs event loop iterations at TRACE level for debugging
- [x] #8 Unit test: event loop processes messages in correct order
- [x] #9 Integration test: event loop handles concurrent peer events
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add event loop chunk to nw/07-client.nw that uses tokio::select\! to multiplex:
   - Peer message reading from active connections (via FuturesUnordered)
   - Request timeout timer (30s interval)
   - Re-announce timer (tracker interval)
   - Keep-alive timer (120s interval)
   - Shutdown signal check
2. Add event loop tests chunk with unit test for message ordering
3. Add integration test chunk for concurrent peer events
4. Update client.rs template to include new chunks
5. Run just lint and just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented main event loop with tokio::select!. Sequential peer reading (not FuturesUnordered) used to avoid external dependency - may optimize later.
<!-- SECTION:NOTES:END -->
