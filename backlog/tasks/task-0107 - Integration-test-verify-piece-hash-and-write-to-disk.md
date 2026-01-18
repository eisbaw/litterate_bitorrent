---
id: task-0107
title: 'Integration test: verify piece hash and write to disk'
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:35'
updated_date: '2026-01-18 02:41'
labels:
  - phase4
  - integration
dependencies:
  - task-0091
  - task-0072
  - task-0099
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create an integration test that exercises the complete Phase 4 pipeline: given raw piece data with a known hash, verify the hash and write to the correct file location. This test validates that all Phase 4 components work together correctly. Use a small test file with known content and hash, simulate receiving blocks out of order, verify hash, and write to disk. This is the exit criteria for Phase 4.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Test creates a small multi-file torrent structure (2-3 files, 2-3 pieces)
- [x] #2 Test simulates receiving blocks for one piece out of order
- [x] #3 Test verifies piece hash matches expected SHA1
- [x] #4 Test writes verified piece to disk at correct location
- [x] #5 Test reads back from disk and verifies content matches original
- [x] #6 Test covers piece spanning two files
- [x] #7 Test covers hash mismatch detection (corrupted data)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan

1. **Add Integration Tests Section to nw/08-disk.nw**
   - Create a new chunk `<<disk integration tests>>` with literate prose
   - Include tests in the module assembly

2. **Test: Multi-file torrent with out-of-order blocks**
   - Create 3 files: file0.bin (80 bytes), file1.bin (60 bytes), file2.bin (60 bytes) = 200 bytes total
   - Use piece_length = 64 bytes (4 pieces total: 64, 64, 64, 8 bytes)
   - Piece 1 spans file0 (bytes 64-79) and file1 (bytes 0-47)
   - Use InProgressPiece to receive blocks out of order
   - Compute expected SHA1 for piece data
   - Verify with verify_piece(), then write with write_piece()
   - Read back and verify content matches

3. **Test: Piece spanning two files**
   - Similar setup to above but focused on piece that crosses file boundary
   - Verify correct file slices are written

4. **Test: Hash mismatch detection**
   - Create piece data with known hash
   - Corrupt one byte
   - Verify verify_piece() returns PieceHashMismatch error

5. **Test utilities**
   - Helper function to compute SHA1 hash of test data
   - Helper to create multi-file test torrent structure

6. **Verification steps**
   - just tangle - extract Rust code
   - just lint - no warnings
   - just test - all tests pass
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented 6 integration tests in nw/08-disk.nw that exercise the complete Phase 4 pipeline:

- **integration_multifile_torrent_out_of_order_blocks**: 3-file torrent (80+60+60=200 bytes), 64-byte pieces, tests piece spanning file boundary with block reception, hash verification, disk write, and read-back verification
- **integration_multiblock_piece_out_of_order**: 40KB piece (3 blocks) received in reverse order (2,0,1), verifies out-of-order block assembly
- **integration_piece_spanning_two_files**: Focused test on piece crossing file boundary (64-byte piece across two 100-byte files)
- **integration_hash_mismatch_detection**: Verifies corrupted data fails hash verification with meaningful error message
- **integration_full_pipeline_multiple_pieces**: Downloads all 4 pieces of torrent in reverse order, writes each, then reads back and reconstructs complete data
- **integration_read_back_spanning_piece**: Tests piece spanning 3 files (30+30+30=90 bytes)

All tests pass: `just tangle && just lint && just test`
<!-- SECTION:NOTES:END -->
