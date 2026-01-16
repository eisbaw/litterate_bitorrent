---
id: task-0069
title: Define tracker response struct and parse bencoded response
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
labels:
  - tracker
  - bencode
  - parsing
dependencies:
  - task-0057
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse the bencoded dictionary returned by the tracker into a structured TrackerResponse. The response contains peers (either compact or dictionary format), interval for re-announce timing, and optional fields. First check for failure reason which indicates tracker-level error. This task handles the bencode-to-struct conversion, not the peer list parsing which is separate.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 TrackerResponse struct with fields: interval, min_interval (optional), peers (raw), complete, incomplete
- [ ] #2 Parse function takes bytes, returns Result<TrackerResponse, TrackerError>
- [ ] #3 If response contains failure reason key, return TrackerErrorResponse with message
- [ ] #4 interval field is required (seconds until next announce)
- [ ] #5 peers field stored as raw bytes initially (parsed by separate task)
- [ ] #6 complete/incomplete (seeders/leechers count) are optional u64
- [ ] #7 Unit test: parse valid compact response
- [ ] #8 Unit test: parse response with failure reason returns error
- [ ] #9 Unit test: parse response with missing interval returns error
<!-- AC:END -->
