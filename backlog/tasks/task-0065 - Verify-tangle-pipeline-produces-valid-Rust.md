---
id: task-0065
title: Verify tangle pipeline produces valid Rust
status: Done
assignee: []
created_date: '2026-01-16 21:30'
updated_date: '2026-01-16 22:39'
labels:
  - verification
  - phase0
dependencies:
  - task-0053
  - task-0035
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
End-to-end verification that 'just tangle' produces a src/main.rs file with the correct generated-code header and compilable Rust code. This validates the noweb-to-Rust pipeline works correctly.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Running 'just tangle' succeeds without errors
- [x] #2 src/main.rs is created in correct location
- [x] #3 src/main.rs contains generated-code header comment
- [x] #4 Header includes 'GENERATED CODE - DO NOT EDIT'
- [x] #5 Header includes source file reference 'nw/00-main.nw'
- [x] #6 Header includes regeneration command 'just tangle'
- [x] #7 The generated code is syntactically valid Rust
<!-- AC:END -->
