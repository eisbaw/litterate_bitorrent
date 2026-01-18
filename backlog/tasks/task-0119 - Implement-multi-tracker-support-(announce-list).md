---
id: task-0119
title: Implement multi-tracker support (announce-list)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 10:22'
labels:
  - feature
  - reliability
  - tracker
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extend tracker module to support announce-list from torrent metadata. Try trackers tier-by-tier and automatically switch if the primary tracker is down. Many torrents provide backup trackers that should be used.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Parse announce-list from torrent metadata (list of lists structure)
- [x] #2 Store tracker URLs in priority order (first tier first)
- [x] #3 On tracker failure, try next tracker in same tier before moving to next tier
- [x] #4 Track which tracker last succeeded for future announces
- [x] #5 Unit test: announce-list parsing with multiple tiers
- [x] #6 Integration test: fallback to secondary tracker when primary fails
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added BEP 12 multi-tracker support:
- announce_list field in Metainfo struct
- TrackerManager with tier-based fallback
- Tracks last successful tracker
- 15 unit tests, 3 integration tests
910 total tests pass.
<!-- SECTION:NOTES:END -->
