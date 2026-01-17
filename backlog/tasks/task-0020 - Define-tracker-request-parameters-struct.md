---
id: task-0020
title: Define tracker request parameters struct
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 08:25'
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
- [x] #1 TrackerRequest struct defined with fields: info_hash, peer_id, port, uploaded, downloaded, left, event, compact
- [x] #2 info_hash is [u8; 20], peer_id is [u8; 20]
- [x] #3 port is u16, uploaded/downloaded/left are u64
- [x] #4 event is Option<TrackerEvent> enum (Started, Stopped, Completed, or None)
- [x] #5 compact is bool (request compact peer format per BEP-23)
- [x] #6 Struct derives Debug and Clone
- [x] #7 Code is added to nw/03-tracker.nw with narrative explaining each field
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Research BEP 3 tracker request parameters
2. Define TrackerEvent enum for the event parameter
3. Define TrackerRequest struct with all required fields
4. Write literate prose explaining each field per BEP 3
5. Add tests verifying struct construction and derive traits
6. Update module structure to include the new definitions
7. Run just check && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented TrackerEvent enum and TrackerRequest struct in nw/03-tracker.nw:

- TrackerEvent enum with Started, Stopped, Completed variants
- TrackerEvent::as_str() method returns protocol strings
- TrackerRequest struct with all BEP 3 parameters:
  - info_hash: [u8; 20]
  - peer_id: [u8; 20]
  - port: u16
  - uploaded, downloaded, left: u64
  - event: Option<TrackerEvent>
  - compact: bool
- Derives Debug and Clone for TrackerRequest
- Derives Debug, Clone, Copy, PartialEq, Eq for TrackerEvent
- Added comprehensive tests for struct construction and derive traits
- Literate prose explains BEP 3 parameters and design rationale

All 181 tests pass including 12 new tests for tracker types.
<!-- SECTION:NOTES:END -->
