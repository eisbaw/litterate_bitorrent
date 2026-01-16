---
id: task-0110
title: Implement info hash computation
status: To Do
assignee: []
created_date: '2026-01-16 21:36'
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
- [ ] #1 Extracts info dict from parsed torrent
- [ ] #2 Re-encodes info dict using bencode encoder
- [ ] #3 Computes SHA1 hash of encoded bytes
- [ ] #4 Returns 20-byte array
- [ ] #5 Info hash matches known values for test torrents
- [ ] #6 Works for both single-file and multi-file formats
- [ ] #7 Unit test with pre-computed expected hash
<!-- AC:END -->
