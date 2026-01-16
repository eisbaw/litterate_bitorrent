---
id: task-0093
title: Verify 'just all' succeeds end-to-end
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
labels:
  - verification
  - phase0
dependencies:
  - task-0074
  - task-0080
  - task-0030
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Final Phase 0 verification: the complete pipeline (tangle + build + weave) runs successfully in a single command. This is the exit criteria for Phase 0 scaffolding.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Running 'just all' completes without errors
- [ ] #2 src/main.rs exists and contains generated header
- [ ] #3 target/debug/literate-bittorrent binary exists and runs
- [ ] #4 doc/literate-bittorrent.pdf exists
- [ ] #5 The binary prints expected output when run
- [ ] #6 The PDF is viewable and contains code
- [ ] #7 just ci passes (tangle + build + test + lint)
<!-- AC:END -->
