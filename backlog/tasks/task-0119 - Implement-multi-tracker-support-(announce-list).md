---
id: task-0119
title: Implement multi-tracker support (announce-list)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 10:19'
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
Implemented multi-tracker support (BEP 12) with the following changes:

## Changes

### nw/02-metainfo.nw
- Added `announce_list: Option<Vec<Vec<String>>>` field to `Metainfo` struct
- Added `parse_announce_list()` function that parses announce-list from torrent metadata
- The function handles all edge cases (malformed data, empty tiers) by returning None
- Added comprehensive unit tests for announce-list parsing

### nw/03-tracker.nw
- Added `TrackerManager` struct that encapsulates tier-based fallback logic
- `TrackerManager::new()` creates a manager from metainfo, using announce-list if present
- `TrackerManager::announce_with_fallback()` tries trackers tier-by-tier
- The manager remembers which tracker last succeeded for efficient subsequent announces
- Added unit tests for TrackerManager construction and attempt ordering

### tests/tracker_integration.rs
- Added integration tests for tracker fallback behavior
- Tests verify that tier 1 is used when tier 0 fails
- Tests verify that successful tracker is remembered for next announce

## Testing
- All unit tests pass (903+ tests)
- All integration tests pass including new tracker fallback tests
- Network tests successfully connect to Ubuntu tracker and demonstrate fallback behavior
<!-- SECTION:NOTES:END -->
