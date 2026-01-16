---
id: task-0004
title: Define BlockRequest struct and request state tracking types
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 21:41'
labels:
  - phase-3
  - phase-5
  - strategy
  - data-structures
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Foundation data structures for the download strategy module. BlockRequest tracks individual block requests with piece index, offset, length, timestamp, and peer identifier. These types are needed before any request management can be implemented. References PRD section on Block Request Tracking.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 BlockRequest struct has fields: piece_index (u32), offset (u32), length (u32), requested_at (Instant), peer (PeerId)
- [ ] #2 BlockRequest derives Debug, Clone for logging and queue operations
- [ ] #3 RequestState enum defines lifecycle: Pending, InFlight, Completed, TimedOut, Cancelled
- [ ] #4 Unit test verifies BlockRequest construction and field access
<!-- AC:END -->
