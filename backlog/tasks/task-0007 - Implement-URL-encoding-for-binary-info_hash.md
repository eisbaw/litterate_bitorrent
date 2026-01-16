---
id: task-0007
title: Implement URL encoding for binary info_hash
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
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
- [ ] #1 URL encoder accepts [u8; 20] and returns percent-encoded String
- [ ] #2 Bytes 0-9, A-Z, a-z, -, _, ., ~ are NOT encoded (RFC 3986 unreserved)
- [ ] #3 All other bytes are percent-encoded as %XX (uppercase hex)
- [ ] #4 Unit test: encoding 20 zero bytes produces %00 repeated 20 times
- [ ] #5 Unit test: encoding known SHA1 hash matches expected output
- [ ] #6 Round-trip test: decoding encoded info_hash yields original bytes
<!-- AC:END -->
