---
id: task-0035
title: Create nw/ directory and master noweb file
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - literate
  - phase0
dependencies:
  - task-0001
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create the literate source directory structure with the master noweb file (00-main.nw). This file defines the document preamble, includes other chapters, and contains a minimal hello-world code chunk to verify the tangle pipeline works.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 nw/ directory exists
- [ ] #2 nw/00-main.nw exists with LaTeX document preamble
- [ ] #3 Document uses LuaLaTeX with fontspec
- [ ] #4 Document includes minted package for syntax highlighting
- [ ] #5 Contains a minimal <<main.rs>> code chunk with hello-world
- [ ] #6 Code chunk includes generated-code header comment
- [ ] #7 notangle -Rmain.rs nw/00-main.nw produces valid Rust code
<!-- AC:END -->
