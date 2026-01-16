---
id: task-0031
title: Implement piece message handler
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:31'
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
- [ ] #1 handle_piece() extracts piece_index, offset, and block data from message
- [ ] #2 Validates block matches a pending request (reject unexpected blocks)
- [ ] #3 Passes block to piece manager for assembly
- [ ] #4 When piece is complete: triggers SHA1 verification
- [ ] #5 On verification success: writes piece to disk, broadcasts have to all peers
- [ ] #6 On verification failure: marks piece for retry, logs warning
- [ ] #7 Updates download progress statistics
- [ ] #8 Unit test: block assembly and piece completion detection
- [ ] #9 Unit test: hash mismatch triggers retry state
<!-- AC:END -->
