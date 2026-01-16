---
id: task-0060
title: Implement main event loop with tokio select
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 21:31'
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
- [ ] #1 Event loop uses tokio::select! to await multiple futures concurrently
- [ ] #2 Handles: peer messages, request timeouts, re-announce timer, keep-alive timer, shutdown signal
- [ ] #3 Routes incoming messages to appropriate handlers
- [ ] #4 Triggers block requests after unchoke or piece completion
- [ ] #5 Replaces disconnected peers from pool
- [ ] #6 Exits cleanly on completion or shutdown signal
- [ ] #7 Logs event loop iterations at TRACE level for debugging
- [ ] #8 Unit test: event loop processes messages in correct order
- [ ] #9 Integration test: event loop handles concurrent peer events
<!-- AC:END -->
