---
id: task-0120
title: Add bandwidth limiting CLI options
status: To Do
assignee: []
created_date: '2026-01-18 09:47'
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
- [ ] #1 Add --max-download-rate flag to CLI args (accepts K/M/G suffixes)
- [ ] #2 Add --max-upload-rate flag to CLI args
- [ ] #3 Implement token bucket rate limiter for download
- [ ] #4 Implement token bucket rate limiter for upload
- [ ] #5 Unit test: rate limiter enforces specified limits
- [ ] #6 Document rate limiting in CLI help and README
<!-- AC:END -->
