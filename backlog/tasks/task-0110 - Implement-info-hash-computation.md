---
id: task-0110
title: Implement info hash computation
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:36'
updated_date: '2026-01-17 00:19'
labels:
  - metainfo
  - crypto
dependencies:
  - task-0078
  - task-0104
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Info hash is SHA1 of the bencoded info dictionary. Critical for torrent identity - used in tracker announces and peer handshakes. Must re-encode the parsed info dict to get canonical byte representation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Extracts info dict from parsed torrent
- [x] #2 Re-encodes info dict using bencode encoder
- [x] #3 Computes SHA1 hash of encoded bytes
- [x] #4 Returns 20-byte array
- [x] #5 Info hash matches known values for test torrents
- [x] #6 Works for both single-file and multi-file formats
- [x] #7 Unit test with pre-computed expected hash
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add get_info_value helper to extract info dict as BencodeValue (not just BTreeMap)
2. Add compute_info_hash function that takes BencodeValue, encodes it, and computes SHA1
3. Update parse function to extract info dict as BencodeValue and compute info_hash
4. Add test using ubuntu.torrent with known hash a1dfefec1a9dd7fa8a041ebeeea271db55126d2f
5. Run tests to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented info hash computation in nw/02-metainfo.nw:

- Added `get_value()` helper to extract info dict as BencodeValue
- Added `compute_info_hash()` function that encodes the info dict and computes SHA1
- Updated `parse()` to compute info_hash instead of using placeholder
- Added comprehensive tests including ubuntu.torrent fixture validation

Key implementation details:
- Uses canonical re-encoding via our bencode encoder (BTreeMap ensures sorted keys)
- SHA1 computed using sha1 crate
- Verified against known ubuntu.torrent hash: a1dfefec1a9dd7fa8a041ebeeea271db55126d2f

All 142 tests pass including new info hash tests.
<!-- SECTION:NOTES:END -->
