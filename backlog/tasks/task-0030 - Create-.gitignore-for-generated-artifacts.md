---
id: task-0030
title: Create .gitignore for generated artifacts
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - infrastructure
  - phase0
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ensure generated files (src/, doc/, target/) are not committed to version control. The nw/ directory is the source of truth; generated files should be reproducible from it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 .gitignore file exists in project root
- [ ] #2 src/ directory is ignored
- [ ] #3 doc/ directory is ignored
- [ ] #4 target/ directory is ignored
- [ ] #5 Cargo.lock is NOT ignored (needed for reproducibility)
- [ ] #6 git status shows no generated files after running just tangle and just build
<!-- AC:END -->
