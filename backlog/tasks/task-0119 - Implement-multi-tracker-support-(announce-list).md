---
id: task-0119
title: Implement multi-tracker support (announce-list)
status: To Do
assignee: []
created_date: '2026-01-18 09:47'
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
- [ ] #1 Parse announce-list from torrent metadata (list of lists structure)
- [ ] #2 Store tracker URLs in priority order (first tier first)
- [ ] #3 On tracker failure, try next tracker in same tier before moving to next tier
- [ ] #4 Track which tracker last succeeded for future announces
- [ ] #5 Unit test: announce-list parsing with multiple tiers
- [ ] #6 Integration test: fallback to secondary tracker when primary fails
<!-- AC:END -->
