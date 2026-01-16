---
id: task-0028
title: Implement Ctrl+C handler for graceful shutdown
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:29'
labels:
  - cli
  - signals
  - shutdown
  - phase7
dependencies:
  - task-0014
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle SIGINT (Ctrl+C) gracefully to allow proper cleanup. Without this, abrupt termination leaves the tracker unnotified (no event=stopped), potentially corrupts partial downloads, and leaves connections hanging. Graceful shutdown is essential for being a good network citizen.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Ctrl+C is intercepted and triggers graceful shutdown sequence
- [ ] #2 Shutdown sequence sends event=stopped to tracker
- [ ] #3 Shutdown sequence closes all peer connections cleanly
- [ ] #4 Shutdown sequence flushes any pending disk writes
- [ ] #5 Progress display shows 'Shutting down...' message on Ctrl+C
- [ ] #6 Second Ctrl+C forces immediate exit (for stuck shutdowns)
- [ ] #7 Exit code is 130 (standard for Ctrl+C termination)
<!-- AC:END -->
