---
id: task-0033
title: Implement user-friendly error reporting
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:29'
labels:
  - errors
  - ux
  - phase7
dependencies:
  - task-0014
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Transform internal errors into user-friendly messages. Technical error types are useful for developers but confusing for users. This layer translates errors like TrackerHttpError into actionable messages like 'Could not connect to tracker: connection refused. Check your network connection.'
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Network errors suggest checking connectivity
- [ ] #2 Invalid torrent file errors identify the specific parsing issue
- [ ] #3 Tracker 'failure reason' responses are displayed verbatim
- [ ] #4 Piece hash mismatch errors identify which piece failed
- [ ] #5 Permission errors suggest checking file/directory permissions
- [ ] #6 All errors exit with non-zero status code
- [ ] #7 Error messages go to stderr, not stdout
<!-- AC:END -->
