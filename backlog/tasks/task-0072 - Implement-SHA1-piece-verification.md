---
id: task-0072
title: Implement SHA1 piece verification
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 09:56'
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
- [x] #1 verify_piece(piece_index, data, expected_hash) computes SHA1 and compares to expected
- [x] #2 Returns Ok(()) on match, Err(PieceHashMismatch) on mismatch
- [x] #3 PieceHashMismatch error includes piece index, expected hash (hex), and computed hash (hex) for debugging
- [x] #4 Hash computation uses sha1 crate
- [x] #5 Unit test with known data and precomputed hash verifies correct behavior
- [x] #6 Unit test with corrupted data verifies hash mismatch is detected
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add piece verification section to 05-pieces.nw with literate prose
2. Implement verify_piece function using sha1 crate
3. Add comprehensive tests for success and hash mismatch cases
4. Tangle to generate Rust code
5. Run check and test to verify implementation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented SHA1 piece verification in 05-pieces.nw with literate prose.

## Changes
- Added new section "Piece Hash Verification" explaining the verification workflow
- Implemented verify_piece(piece_index, data, expected_hash) function
- Uses sha1 crate (Sha1::new, update, finalize)
- Returns Ok(()) on match, Err(PieceError::PieceHashMismatch) on mismatch
- Error includes piece_index, expected_hash (hex), and computed_hash (hex)

## Tests Added
- verify_piece_succeeds_with_correct_data: uses known SHA1 of "test"
- verify_piece_fails_with_corrupted_data: verifies hash mismatch error details
- verify_piece_fails_with_empty_data: empty data vs non-empty hash
- verify_piece_succeeds_with_empty_data_and_correct_hash: SHA1 of empty string
- verify_piece_piece_index_preserved_in_error: tests various piece indices
- verify_piece_with_larger_data: tests 256 KiB piece (typical size)

## Verification
- All 403 tests pass including 6 new piece verification tests
- Doc tests pass (verify_piece example included in function docs)
<!-- SECTION:NOTES:END -->
