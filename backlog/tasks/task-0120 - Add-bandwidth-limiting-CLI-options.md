---
id: task-0120
title: Add bandwidth limiting CLI options
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 10:33'
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
## Implementation Summary

Added bandwidth limiting CLI options with token bucket rate limiter.

### CLI Changes (nw/06-cli.nw)
- Added `--max-download-rate` flag accepting K/M/G suffixes (e.g., 1M, 500K)
- Added `--max-upload-rate` flag with same format
- Implemented `parse_rate_limit()` function to convert human-readable rates to bytes/sec
- Added comprehensive tests for rate limit parsing

### Token Bucket Rate Limiter (nw/07-client.nw)
- Implemented `TokenBucket` struct with async `acquire()` method
- Uses lazy refill strategy - tokens calculated on-demand based on elapsed time
- Bucket capacity equals rate (1 second of burst allowed)
- Thread-safe using tokio::sync::Mutex
- `try_acquire()` for non-blocking scenarios
- Added 10 unit tests for rate limiter behavior

### Files Modified
- `nw/06-cli.nw`: CLI args, rate parser, tests
- `nw/07-client.nw`: TokenBucket implementation and tests

### Testing
- All 938 tests pass
- Clippy lint passes with no warnings
<!-- SECTION:NOTES:END -->
