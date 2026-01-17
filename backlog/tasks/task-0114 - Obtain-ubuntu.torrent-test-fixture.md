---
id: task-0114
title: Obtain ubuntu.torrent test fixture
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:38'
updated_date: '2026-01-16 23:15'
labels:
  - testing
  - fixture
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Download a stable Ubuntu ISO torrent file to use as integration test fixture. Choose an LTS version for stability. Store in tests/fixtures/ and document the expected hash values for verification.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tests/fixtures/ directory exists
- [x] #2 ubuntu.torrent file downloaded from official Ubuntu source
- [x] #3 Document expected info_hash (hex) in a comment or test
- [x] #4 Document expected piece_count
- [x] #5 Document expected total_length
- [x] #6 File committed to repository
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Ensure tests/fixtures/ exists (from task-87)
2. Download Ubuntu 24.04 LTS torrent from releases.ubuntu.com
3. Parse torrent to extract info_hash, piece_count, total_length
4. Document these values in a test file or fixture metadata
5. Verify file is valid and commit
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Downloaded Ubuntu 24.04.3 LTS server ISO torrent from https://releases.ubuntu.com/24.04/

Documented metadata in tests/fixtures/ubuntu.torrent.expected:
- INFO_HASH: a1dfefec1a9dd7fa8a041ebeeea271db55126d2f
- PIECE_COUNT: 12602
- TOTAL_LENGTH: 3303444480 (3.08 GiB)
- PIECE_LENGTH: 262144 (256 KiB)

These values were extracted by parsing the torrent and computing SHA1 of the bencoded info dict.
<!-- SECTION:NOTES:END -->
