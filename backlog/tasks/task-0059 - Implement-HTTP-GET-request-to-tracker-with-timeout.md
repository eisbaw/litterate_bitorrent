---
id: task-0059
title: Implement HTTP GET request to tracker with timeout
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
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
- [ ] #1 Async function takes announce URL, returns Result<Vec<u8>, TrackerError>
- [ ] #2 Uses reqwest client with 30 second timeout configured
- [ ] #3 HTTP 200 response body is returned as bytes
- [ ] #4 Non-2xx HTTP status returns appropriate error
- [ ] #5 Connection/DNS failures wrapped in TrackerHttpError
- [ ] #6 Timeout returns TrackerTimeout error
- [ ] #7 Function follows redirects (trackers sometimes redirect)
- [ ] #8 User-Agent header set to client identifier (e.g., LiterateTorrent/0.1)
<!-- AC:END -->
