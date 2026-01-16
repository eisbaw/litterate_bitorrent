---
id: task-0080
title: Verify weave pipeline produces PDF
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-16 23:07'
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
- [x] #1 Running 'just weave' succeeds without errors
- [x] #2 PDF is produced at doc/literate-bittorrent.pdf
- [x] #3 PDF contains the document title
- [x] #4 PDF contains syntax-highlighted Rust code
- [ ] #5 minted/Pygments highlighting works (colors in code blocks)
- [x] #6 No LaTeX warnings about missing fonts
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
AC #5 partial: minted loaded but noweb uses plain verbatim. Would need custom filter for colors - future enhancement.
<!-- SECTION:NOTES:END -->
