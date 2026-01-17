---
id: task-0059
title: Implement HTTP GET request to tracker with timeout
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 08:34'
labels:
  - tracker
  - network
  - async
dependencies:
  - task-0041
  - task-0057
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Send the announce request to the tracker and receive the response. Use reqwest with async/await and a 30-second timeout as specified in PRD. The response body is raw bytes (bencoded dict). Handle HTTP-level errors (connection refused, DNS failure, 4xx/5xx responses) distinctly from tracker protocol errors. This is the network boundary of the tracker module.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Async function takes announce URL, returns Result<Vec<u8>, TrackerError>
- [x] #2 Uses reqwest client with 30 second timeout configured
- [x] #3 HTTP 200 response body is returned as bytes
- [x] #4 Non-2xx HTTP status returns appropriate error
- [x] #5 Connection/DNS failures wrapped in TrackerHttpError
- [x] #6 Timeout returns TrackerTimeout error
- [x] #7 Function follows redirects (trackers sometimes redirect)
- [x] #8 User-Agent header set to client identifier (e.g., LiterateTorrent/0.1)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add async announce_http function to nw/03-tracker.nw
2. Configure reqwest client with 30s timeout and User-Agent header
3. Map reqwest errors to TrackerError variants (timeout -> TrackerTimeout, other -> TrackerHttpError)
4. Handle non-2xx status codes as TrackerHttpError
5. Enable redirect following (reqwest default)
6. Add literate prose explaining HTTP request lifecycle
7. Add unit tests for error mapping
8. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented async HTTP GET request function for tracker announces in nw/03-tracker.nw.

Key implementation details:
- Added `announce_http(url: &str) -> Result<Vec<u8>, TrackerError>` async function
- Configured reqwest client with 30-second timeout per PRD specification
- Set User-Agent header to "LiterateTorrent/0.1" for client identification
- Redirect following enabled via reqwest default policy
- Timeout errors detected via `reqwest::Error::is_timeout()` and mapped to `TrackerTimeout`
- Non-2xx status codes converted to `TrackerHttpError` via `error_for_status()`
- Connection/DNS failures wrapped in `TrackerHttpError` automatically via `#[from]`

Added literate prose explaining:
- HTTP request lifecycle for tracker announces
- Design decisions (client-per-request, timeout coverage, error mapping)
- Why redirect following is important for load balancing

Tests added:
- `user_agent_is_set` - verifies USER_AGENT constant
- `tracker_timeout_is_30_seconds` - verifies TRACKER_TIMEOUT_SECS constant

Verified with `just check && just test && just lint` - all 192 tests pass, no warnings.
<!-- SECTION:NOTES:END -->
