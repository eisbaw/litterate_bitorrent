---
id: task-0113
title: Integration test with real ubuntu.torrent file
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:37'
updated_date: '2026-01-17 00:29'
labels:
  - metainfo
  - integration
  - phase1
dependencies:
  - task-0114
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Verify complete Phase 1 implementation by parsing a real torrent file. This is the exit criteria validation: load tests/fixtures/ubuntu.torrent, print info hash, piece count, and total size. Confirms end-to-end correctness.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ubuntu.torrent file exists in tests/fixtures/
- [x] #2 Successfully decode the bencode
- [x] #3 Successfully parse Metainfo from bencoded dict
- [x] #4 Print info hash in hex format
- [x] #5 Print correct piece count (total_length / piece_length rounded up)
- [x] #6 Print total size in bytes and human-readable format
- [x] #7 All values match known correct values for the torrent
- [x] #8 Test passes in CI
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing test info_hash_ubuntu_torrent (already in place)
2. Enhance test to print values (info hash hex, piece count, total size in bytes and human-readable)
3. Add helper for human-readable size formatting
4. Run tests to verify all passes
5. Mark all ACs as complete
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added integration test `integration_test_ubuntu_torrent` to nw/02-metainfo.nw.

The test validates Phase 1 exit criteria by:
- Verifying ubuntu.torrent fixture exists
- Successfully decoding bencode and parsing Metainfo
- Printing info hash (a1dfefec1a9dd7fa8a041ebeeea271db55126d2f)
- Printing piece count (12602)
- Printing total size in bytes (3303444480) and human-readable format (3.08 GiB)
- Asserting all values match expected values from tests/fixtures/ubuntu.torrent.expected

Also added helper functions:
- `hex_string()` - formats 20-byte hash as hex string
- `human_readable_size()` - formats bytes as GiB/MiB/KiB/bytes

All 154 tests pass. Clippy clean.
<!-- SECTION:NOTES:END -->
