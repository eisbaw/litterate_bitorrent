---
id: task-0004
title: Define BlockRequest struct and request state tracking types
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 09:46'
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
- [x] #1 BlockRequest struct has fields: piece_index (u32), offset (u32), length (u32), requested_at (Instant), peer (PeerId)
- [x] #2 BlockRequest derives Debug, Clone for logging and queue operations
- [x] #3 RequestState enum defines lifecycle: Pending, InFlight, Completed, TimedOut, Cancelled
- [x] #4 Unit test verifies BlockRequest construction and field access
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define PeerId type alias (20-byte peer identifier)
2. Define BlockRequest struct with piece_index, offset, length, requested_at, peer fields
3. Define RequestState enum for lifecycle tracking
4. Add unit tests for BlockRequest construction and field access
5. Write literate prose explaining the data structures
6. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented BlockRequest struct and RequestState enum in the pieces module (05-pieces.nw).

Changes:
- Added PeerId type alias ([u8; 20]) for peer identification
- Added BlockRequest struct with: piece_index (u32), offset (u32), length (u32), requested_at (Instant), peer (PeerId)
- BlockRequest derives Debug, Clone and includes helper methods: new(), is_timed_out(), age(), matches()
- Added RequestState enum with lifecycle states: Pending, InFlight, Completed, TimedOut, Cancelled
- RequestState derives Debug, Clone, Copy, PartialEq, Eq and includes helper methods: is_terminal(), is_active(), can_retry()
- Added comprehensive unit tests for both types (13 new tests total)
- All 379 unit tests pass including doc tests

The literate prose explains the purpose of block request tracking and documents the state machine transitions.
<!-- SECTION:NOTES:END -->
