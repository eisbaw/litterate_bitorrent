---
id: task-0080
title: Verify weave pipeline produces PDF
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
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
End-to-end verification that 'just weave' produces a valid PDF document. This validates the LuaLaTeX toolchain, minted syntax highlighting, and document structure all work together.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Running 'just weave' succeeds without errors
- [ ] #2 PDF is produced at doc/literate-bittorrent.pdf
- [ ] #3 PDF contains the document title
- [ ] #4 PDF contains syntax-highlighted Rust code
- [ ] #5 minted/Pygments highlighting works (colors in code blocks)
- [ ] #6 No LaTeX warnings about missing fonts
<!-- AC:END -->
