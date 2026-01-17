---
id: task-0007
title: Implement URL encoding for binary info_hash
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 08:15'
labels:
  - tracker
  - encoding
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The info_hash is 20 bytes of binary data that must be URL-encoded for the tracker announce URL. Standard percent-encoding libraries may not handle raw binary correctly. This is a foundational utility needed before constructing tracker requests. The encoding must handle all byte values 0x00-0xFF, not just ASCII. This is a common source of bugs in BitTorrent clients.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 URL encoder accepts [u8; 20] and returns percent-encoded String
- [x] #2 Bytes 0-9, A-Z, a-z, -, _, ., ~ are NOT encoded (RFC 3986 unreserved)
- [x] #3 All other bytes are percent-encoded as %XX (uppercase hex)
- [x] #4 Unit test: encoding 20 zero bytes produces %00 repeated 20 times
- [x] #5 Unit test: encoding known SHA1 hash matches expected output
- [x] #6 Round-trip test: decoding encoded info_hash yields original bytes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze RFC 3986 unreserved characters for URL encoding
2. Implement url_encode function accepting [u8; 20] and returning percent-encoded String
3. Write literate prose explaining why binary data needs percent-encoding
4. Add unit tests: all-zeros, known SHA1 hash, special characters
5. Add round-trip test using percent decoding
6. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented RFC 3986 percent-encoding for 20-byte binary data (info_hash, peer_id).

- Added `url_encode()` function that accepts `&[u8; 20]` and returns percent-encoded String
- Unreserved characters (A-Z, a-z, 0-9, -, _, ., ~) pass through unchanged
- All other bytes are encoded as %XX with uppercase hex digits
- Pre-allocates worst-case capacity (60 chars) for efficiency

Literate prose explains:
- Why binary data needs percent-encoding in tracker URLs
- Which characters are unreserved per RFC 3986
- Why this is a common source of bugs in BitTorrent clients

6 comprehensive tests added:
1. All-zeros encoding produces %00 repeated 20 times
2. Unreserved characters pass through without encoding
3. Known SHA1 hash verifies realistic encoding behavior
4. URL-significant special characters are properly encoded
5. High bytes (0x80-0xFF) are correctly handled
6. Round-trip test verifies decode(encode(x)) == x

All 165 tests pass, including new URL encoding tests. No clippy warnings.
<!-- SECTION:NOTES:END -->
