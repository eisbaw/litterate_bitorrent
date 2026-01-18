---
id: task-0120
title: Add bandwidth limiting CLI options
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 10:44'
labels:
  - feature
  - cli
  - client
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add CLI flags for upload/download speed limits (e.g., --max-download-rate 1M --max-upload-rate 500K). This prevents the client from saturating home internet connections and allows background downloading.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Add --max-download-rate flag to CLI args (accepts K/M/G suffixes)
- [x] #2 Add --max-upload-rate flag to CLI args
- [x] #3 Implement token bucket rate limiter for download
- [x] #4 Implement token bucket rate limiter for upload
- [x] #5 Unit test: rate limiter enforces specified limits
- [x] #6 Document rate limiting in CLI help and README
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added --max-download-rate and --max-upload-rate CLI flags.
Token bucket rate limiter with lazy refill.
28 unit tests total.
Note: Request size must be <= bucket capacity (satisfied by BitTorrent block sizes).
<!-- SECTION:NOTES:END -->
