---
id: task-0050
title: Implement re-announce timer
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 22:46'
labels:
  - phase-6
  - tracker
  - networking
dependencies:
  - task-0006
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implements periodic tracker re-announces. The tracker response includes an interval (typically 30 minutes). We must re-announce to get fresh peers and update our statistics (uploaded/downloaded/left).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Parse and store interval from tracker response
- [x] #2 Schedule re-announce after interval elapses
- [x] #3 Re-announce includes updated downloaded/uploaded/left statistics
- [x] #4 Handle tracker failure gracefully (retry with backoff, continue with existing peers)
- [x] #5 Merge new peers into available pool without duplicates
- [x] #6 Logs re-announce at INFO level, failures at WARN
- [x] #7 Unit test: re-announce scheduled correctly based on interval
- [x] #8 Integration test: handles tracker temporary unavailability
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented re-announce timer with the following features:

- Extended EventLoopContext with metainfo, info_hash, port, and re_announce_failures fields
- Added handle_re_announce() function that:
  - Builds TrackerRequest with current download stats (bytes_verified, left)
  - Calls announce() and adds new peers to connection manager
  - Logs success at INFO level with stats (peers received/added, interval, seeders/leechers)
  - Logs failures at WARN level with backoff info
- Implemented exponential backoff for tracker failures:
  - Capped at MAX_RE_ANNOUNCE_BACKOFF_MULTIPLIER (8)
  - Reset on success
- Added unit tests for backoff logic and tracker unavailability handling

Files modified:
- nw/07-client.nw: EventLoopContext, handle_re_announce(), tests
<!-- SECTION:NOTES:END -->
