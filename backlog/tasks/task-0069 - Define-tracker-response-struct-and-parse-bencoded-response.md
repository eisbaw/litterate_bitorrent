---
id: task-0069
title: Define tracker response struct and parse bencoded response
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 08:40'
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
- [x] #1 TrackerResponse struct with fields: interval, min_interval (optional), peers (raw), complete, incomplete
- [x] #2 Parse function takes bytes, returns Result<TrackerResponse, TrackerError>
- [x] #3 If response contains failure reason key, return TrackerErrorResponse with message
- [x] #4 interval field is required (seconds until next announce)
- [x] #5 peers field stored as raw bytes initially (parsed by separate task)
- [x] #6 complete/incomplete (seeders/leechers count) are optional u64
- [x] #7 Unit test: parse valid compact response
- [x] #8 Unit test: parse response with failure reason returns error
- [x] #9 Unit test: parse response with missing interval returns error
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze tracker response format per BEP 3 and BEP 23
2. Define TrackerResponse struct with required/optional fields
3. Implement parse_tracker_response function
4. Handle failure reason check first (returns TrackerErrorResponse)
5. Handle missing interval (returns TrackerInvalidResponse)
6. Store peers as raw bytes (separate task handles parsing)
7. Write literate prose explaining the response format
8. Add comprehensive unit tests
9. Verify with nix-shell --run "just check && just test"
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented TrackerResponse struct and parse_tracker_response function in nw/03-tracker.nw.

## Changes
- Added TrackerResponse struct with interval (required), min_interval (optional), peers (raw bytes), complete (optional), incomplete (optional)
- Added parse_tracker_response function that:
  - Decodes bencode and validates dictionary structure
  - Checks for failure reason first (returns TrackerErrorResponse)
  - Extracts required interval field (returns TrackerInvalidResponse if missing/invalid)
  - Handles both compact (BEP 23) and dictionary format peers
  - Extracts optional fields: min_interval, complete, incomplete
- Added literate prose explaining BEP 3 tracker response format
- Added 14 comprehensive unit tests covering success and error cases

## Testing
- All 207 tests pass
- Tests cover: valid compact response, all optional fields, failure reason handling, missing required fields, invalid types, dictionary format peers, negative values
<!-- SECTION:NOTES:END -->
