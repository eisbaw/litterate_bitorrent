---
id: task-0041
title: Construct announce URL with query parameters
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - tracker
  - url
dependencies:
  - task-0007
  - task-0020
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build the complete tracker announce URL from the base announce URL (from metainfo) and request parameters. This involves URL-encoding the binary info_hash and peer_id, then appending all parameters as query string. The URL construction must be correct for the tracker to return peers. Order of parameters should not matter but canonical ordering helps debugging.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Function takes base announce URL and TrackerRequest, returns full URL string
- [ ] #2 info_hash is URL-encoded using the binary URL encoder from task-7
- [ ] #3 peer_id is URL-encoded similarly
- [ ] #4 Numeric parameters (port, uploaded, downloaded, left) are formatted as decimal strings
- [ ] #5 event parameter is only included if not None (started, stopped, completed)
- [ ] #6 compact=1 is included when compact is true
- [ ] #7 Unit test: constructed URL matches expected format with known inputs
- [ ] #8 Unit test: handles announce URL that already has query parameters (use & not ?)
<!-- AC:END -->
