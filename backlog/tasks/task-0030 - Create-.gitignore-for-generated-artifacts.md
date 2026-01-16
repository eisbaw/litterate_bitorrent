---
id: task-0030
title: Create .gitignore for generated artifacts
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 22:24'
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
- [x] #1 .gitignore file exists in project root
- [x] #2 src/ directory is ignored
- [x] #3 doc/ directory is ignored
- [x] #4 target/ directory is ignored
- [x] #5 Cargo.lock is NOT ignored (needed for reproducibility)
- [x] #6 git status shows no generated files after running just tangle and just build
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create .gitignore file in project root
2. Add entries for src/, doc/, target/ directories
3. Add explicit negation for Cargo.lock
4. Verify file is correct
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created .gitignore file in project root that:
- Ignores src/ (tangled code from noweb)
- Ignores doc/ (woven PDF documentation)
- Ignores target/ (Cargo build output)
- Explicitly does NOT ignore Cargo.lock via negation pattern

Verified by creating test files in each directory and confirming git status does not show them as untracked.
<!-- SECTION:NOTES:END -->
