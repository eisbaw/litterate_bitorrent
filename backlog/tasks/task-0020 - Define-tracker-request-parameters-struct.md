---
id: task-0020
title: Define tracker request parameters struct
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - tracker
  - data-structure
dependencies:
  - task-0007
  - task-0013
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Before constructing announce URLs, we need a struct that captures all tracker request parameters. This provides a clean data structure for the announce operation and makes the code self-documenting. The struct will be used to build query strings and also serves as documentation of what parameters the BitTorrent tracker protocol requires.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 TrackerRequest struct defined with fields: info_hash, peer_id, port, uploaded, downloaded, left, event, compact
- [ ] #2 info_hash is [u8; 20], peer_id is [u8; 20]
- [ ] #3 port is u16, uploaded/downloaded/left are u64
- [ ] #4 event is Option<TrackerEvent> enum (Started, Stopped, Completed, or None)
- [ ] #5 compact is bool (request compact peer format per BEP-23)
- [ ] #6 Struct derives Debug and Clone
- [ ] #7 Code is added to nw/03-tracker.nw with narrative explaining each field
<!-- AC:END -->
