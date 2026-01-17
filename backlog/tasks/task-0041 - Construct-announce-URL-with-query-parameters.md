---
id: task-0041
title: Construct announce URL with query parameters
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 08:29'
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
- [x] #1 Function takes base announce URL and TrackerRequest, returns full URL string
- [x] #2 info_hash is URL-encoded using the binary URL encoder from task-7
- [x] #3 peer_id is URL-encoded similarly
- [x] #4 Numeric parameters (port, uploaded, downloaded, left) are formatted as decimal strings
- [x] #5 event parameter is only included if not None (started, stopped, completed)
- [x] #6 compact=1 is included when compact is true
- [x] #7 Unit test: constructed URL matches expected format with known inputs
- [x] #8 Unit test: handles announce URL that already has query parameters (use & not ?)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add build_announce_url function with signature: fn build_announce_url(announce_url: &str, request: &TrackerRequest) -> String
2. Detect if announce_url already has query params (contains ?) to choose ? or & as separator
3. URL-encode info_hash and peer_id using existing url_encode function
4. Format numeric params (port, uploaded, downloaded, left) as decimal strings
5. Conditionally include event param only if Some
6. Include compact=1 when compact is true
7. Write literate prose explaining BEP 3 URL construction
8. Add chunk references to tracker.rs module structure
9. Add tests for: known inputs, URL with existing query params, all event types, compact true/false
10. Verify with nix-shell --run "just check && just test"
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented build_announce_url function in nw/03-tracker.nw with:

- Function signature: pub fn build_announce_url(announce_url: &str, request: &TrackerRequest) -> String
- Uses existing url_encode function for info_hash and peer_id binary parameters
- Detects if announce URL already has query params (contains ?) to choose ? or & as separator
- Formats port, uploaded, downloaded, left as decimal strings
- Conditionally includes event parameter only when Some
- Includes compact=1 only when compact is true
- Pre-allocates String capacity for efficiency

Added literate prose explaining BEP 3 URL construction with 5 numbered steps.

Added 9 unit tests covering:
- Basic URL construction
- URLs with existing query parameters (uses & not ?)
- Without event parameter
- Without compact flag
- stopped and completed events
- Binary encoding verification
- Large u64 values
- Known output comparison

All 190 tests pass including the 9 new build_announce_url tests.
<!-- SECTION:NOTES:END -->
