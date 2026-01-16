---
id: task-0024
title: Define PieceState enum
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - phase4
  - foundation
dependencies:
  - task-0017
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define the PieceState enum that tracks the download state of each piece. This is the core state machine for piece management. The states are: Missing (not started), InProgress (some blocks received, tracked via BitVec), Verified (hash matched, written to disk), and Failed (hash mismatch, will retry). The InProgress variant carries a BitVec to track which blocks have been received, enabling resumption and detecting duplicate blocks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PieceState enum has variants: Missing, InProgress, Verified, Failed
- [ ] #2 InProgress variant contains blocks_received: BitVec tracking which blocks are present
- [ ] #3 Failed variant is distinct from Missing to allow tracking retry attempts
- [ ] #4 State transitions documented: Missing->InProgress (first block), InProgress->Verified (hash ok), InProgress->Failed (hash mismatch), Failed->InProgress (retry)
- [ ] #5 PieceState implements Clone and Debug
<!-- AC:END -->
