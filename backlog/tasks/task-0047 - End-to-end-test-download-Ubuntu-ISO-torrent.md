---
id: task-0047
title: 'End-to-end test: download Ubuntu ISO torrent'
status: In Progress
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-18 03:58'
labels:
  - testing
  - e2e
  - phase7
dependencies:
  - task-0025
  - task-0028
  - task-0033
  - task-0021
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The ultimate validation: download a real torrent from start to finish. This proves the entire system works together - bencode parsing, tracker communication, peer protocol, piece management, and disk I/O. Ubuntu ISO is ideal: well-seeded, legal, large enough to test real behavior, small enough to be practical.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Download starts with: literate-bittorrent ubuntu.torrent -o ./downloads
- [x] #2 Progress display shows pieces completing over time
- [ ] #3 Download completes successfully (all pieces verified)
- [ ] #4 Downloaded file matches expected SHA256 hash
- [x] #5 Ctrl+C during download triggers graceful shutdown
- [x] #6 Resume after interruption works (does not re-download verified pieces)
- [x] #7 Test is added to justfile as 'just e2e' recipe
- [x] #8 Test can run in CI (or is marked as manual with instructions)
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Implementation Summary

### Main Program Integration (nw/00-main.nw)
- Connected CLI arg parsing to torrent loading, tracker announce, peer connections
- Implemented run_download() orchestration function
- Added connect_initial_peers() for establishing connections
- Added check_resume() for verifying existing pieces on startup
- Integrated event loop with progress display and shutdown handling

### Resume Support (nw/08-disk.nw)
- Added read_piece() function for reading existing pieces
- Added DiskError::ReadError variant
- Resume checks SHA1 hash of existing pieces before marking verified

### CLI Error Handling (nw/06-cli.nw)
- Added ReadError formatting for resume operation failures

### Justfile Recipes
- just e2e - runs integration tests with network access
- just download - manual download with ubuntu.torrent
- just download-to <path> - download to custom directory

### Tests Added
- tests/cli_integration.rs - CLI argument parsing tests (8 tests)
- Existing download_integration.rs handles full e2e download

### Notes
- AC#3 and AC#4 require actual download completion (network dependent)
- The e2e test can fail due to tracker/peer availability
<!-- SECTION:NOTES:END -->
