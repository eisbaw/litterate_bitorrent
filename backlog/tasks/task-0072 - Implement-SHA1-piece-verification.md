---
id: task-0072
title: Implement SHA1 piece verification
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
labels:
  - phase4
  - verification
dependencies:
  - task-0062
  - task-0008
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement SHA1 hash verification for completed pieces. When all blocks of a piece have been received, compute the SHA1 hash of the buffered data and compare against the expected hash from the torrent metainfo. If the hash matches, the piece is verified and ready for writing to disk. If the hash does not match, the piece failed verification and must be cleared for retry. This is the integrity check that ensures downloaded data is correct.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 verify_piece(piece_index, data, expected_hash) computes SHA1 and compares to expected
- [ ] #2 Returns Ok(()) on match, Err(PieceHashMismatch) on mismatch
- [ ] #3 PieceHashMismatch error includes piece index, expected hash (hex), and computed hash (hex) for debugging
- [ ] #4 Hash computation uses sha1 crate
- [ ] #5 Unit test with known data and precomputed hash verifies correct behavior
- [ ] #6 Unit test with corrupted data verifies hash mismatch is detected
<!-- AC:END -->
