---
id: task-0008
title: Define piece error types
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - phase4
  - foundation
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create error types for piece management operations. These errors need clear context for debugging hash mismatches, disk failures, and geometry violations. Following fail-fast principles, each error must include enough information to diagnose the problem without additional logging.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PieceHashMismatch error includes piece index, expected hash (hex), and computed hash (hex)
- [ ] #2 InvalidPieceIndex error includes the invalid index and the valid range
- [ ] #3 InvalidBlockOffset error includes piece index, offset, and piece size
- [ ] #4 DiskWriteError wraps std::io::Error with file path and byte offset context
- [ ] #5 All errors implement std::error::Error and Display with human-readable messages
<!-- AC:END -->
