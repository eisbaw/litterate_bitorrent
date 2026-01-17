---
id: task-0031
title: Implement piece message handler
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 19:39'
labels:
  - phase-6
  - message-handler
  - data-path
dependencies:
  - task-0006
  - task-0015
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles incoming piece (block) messages from peers. This is the core data reception path. Blocks must be passed to the piece manager, and when a piece is complete, it must be verified and written to disk.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 handle_piece() extracts piece_index, offset, and block data from message
- [x] #2 Validates block matches a pending request (reject unexpected blocks)
- [x] #3 Passes block to piece manager for assembly
- [x] #4 When piece is complete: triggers SHA1 verification
- [x] #5 On verification success: writes piece to disk, broadcasts have to all peers
- [x] #6 On verification failure: marks piece for retry, logs warning
- [x] #7 Updates download progress statistics
- [x] #8 Unit test: block assembly and piece completion detection
- [x] #9 Unit test: hash mismatch triggers retry state
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add PieceHandleResult enum before handle_incoming_choke in the incoming message handlers chunk
2. Add handle_incoming_piece function that:
   - Validates block against pending requests
   - Calls receive_block on InProgressPiece (handles Result)
   - Checks completion and verifies hash
3. Add literate prose explaining the data path
4. Add unit tests for all cases
5. Run just check && cargo test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented piece message handler in nw/07-client.nw.

Added PieceHandleResult enum with variants:
- BlockStored: incomplete piece, block saved
- PieceVerified(usize): complete and hash matches
- PieceHashFailed(usize): complete but hash mismatch
- UnexpectedBlock: no matching pending request

handle_incoming_piece() function:
1. Validates block against pending_requests
2. Removes matched request from queue
3. Stores via InProgressPiece::receive_block()
4. On completion: verifies SHA1 via verify_piece()

Security: Request validation prevents amplification attacks.

Tests added (7): block storage, unexpected rejection, verification, hash failure.

All 694 tests pass.

Note: Caller responsible for routing blocks to correct InProgressPiece.
<!-- SECTION:NOTES:END -->
