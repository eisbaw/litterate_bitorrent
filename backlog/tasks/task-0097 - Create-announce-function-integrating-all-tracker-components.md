---
id: task-0097
title: Create announce function integrating all tracker components
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
labels:
  - tracker
  - integration
dependencies:
  - task-0059
  - task-0090
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
High-level async function that performs a complete tracker announce: constructs URL, makes HTTP request, parses response, extracts peers. This is the main entry point for the tracker module. Returns a clean AnnounceResult with peers and metadata. Handles all error cases by mapping to appropriate TrackerError variants.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Async function takes Metainfo, TrackerRequest, returns Result<AnnounceResult, TrackerError>
- [ ] #2 AnnounceResult struct contains: peers (Vec<SocketAddr>), interval (u64), seeders/leechers (Option<u64>)
- [ ] #3 Internally calls URL construction, HTTP request, response parsing, peer extraction
- [ ] #4 Logs announce URL at debug level, result summary at info level
- [ ] #5 Errors from any stage propagate with appropriate type
- [ ] #6 Function documented in nw/03-tracker.nw with narrative flow
<!-- AC:END -->
