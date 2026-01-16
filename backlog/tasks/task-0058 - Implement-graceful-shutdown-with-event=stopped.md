---
id: task-0058
title: Implement graceful shutdown with event=stopped
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - shutdown
  - lifecycle
dependencies:
  - task-0006
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles clean shutdown on Ctrl+C or download completion. Sends event=stopped to tracker, closes all peer connections gracefully, and ensures any buffered data is written to disk. Critical for good swarm citizenship.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Register Ctrl+C (SIGINT) handler using tokio signal
- [ ] #2 On shutdown signal: set shutdown flag, stop accepting new work
- [ ] #3 Send event=stopped to tracker (best effort, with 5 second timeout)
- [ ] #4 Close all peer connections with proper TCP shutdown
- [ ] #5 Flush any pending disk writes
- [ ] #6 Log shutdown at INFO level
- [ ] #7 Exit with code 0 on successful shutdown, non-zero on error
- [ ] #8 Unit test: shutdown flag stops event loop
- [ ] #9 Integration test: Ctrl+C triggers clean shutdown sequence
<!-- AC:END -->
