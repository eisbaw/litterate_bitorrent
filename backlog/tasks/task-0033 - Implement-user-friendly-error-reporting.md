---
id: task-0033
title: Implement user-friendly error reporting
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 01:33'
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
- [x] #1 Network errors suggest checking connectivity
- [x] #2 Invalid torrent file errors identify the specific parsing issue
- [x] #3 Tracker 'failure reason' responses are displayed verbatim
- [x] #4 Piece hash mismatch errors identify which piece failed
- [x] #5 Permission errors suggest checking file/directory permissions
- [x] #6 All errors exit with non-zero status code
- [x] #7 Error messages go to stderr, not stdout
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented user-friendly error reporting:
- ClientRunError wraps all subsystem errors
- format_user_error() provides actionable messages
- Individual formatters for each error category
- Network errors suggest checking connectivity
- Piece errors identify failed piece number
- All errors exit with code 1 to stderr
<!-- SECTION:NOTES:END -->
