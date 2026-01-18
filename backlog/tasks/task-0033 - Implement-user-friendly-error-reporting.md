---
id: task-0033
title: Implement user-friendly error reporting
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 01:29'
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
Implemented user-friendly error reporting module in nw/06-cli.nw

## Summary

- Added ClientRunError enum that wraps all subsystem errors (metainfo, tracker, disk, pieces, peer, client, I/O)
- Implemented format_user_error() function that translates technical errors to user-friendly messages
- Added individual formatters for each error category:
  - format_torrent_error: suggests re-downloading corrupt torrent files
  - format_tracker_error: handles HTTP errors, timeout, and displays tracker failure reasons verbatim (AC#3)
  - format_disk_error: suggests checking permissions for EACCES errors (AC#5)
  - format_piece_error: identifies which piece failed verification (AC#4)
  - format_peer_error: explains peer communication issues
  - format_io_error: generic I/O error handling
- Added exit_with_error() helper that prints to stderr and exits with code 1 (AC#6, AC#7)
- Added comprehensive tests for error formatting

## Design Decisions

- Error messages follow three principles: simple language, actionable guidance, relevant details
- Tracker failure reasons are displayed verbatim since trackers write user-facing messages
- Permission errors specifically suggest checking file/directory permissions
- Network errors suggest checking connectivity

## Testing

- just tangle: PASS
- just lint: PASS
- just test: 93 passed, 0 failed
<!-- SECTION:NOTES:END -->
