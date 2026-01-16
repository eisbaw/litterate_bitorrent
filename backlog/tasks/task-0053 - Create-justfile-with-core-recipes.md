---
id: task-0053
title: Create justfile with core recipes
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
labels:
  - infrastructure
  - phase0
dependencies:
  - task-0001
  - task-0035
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create the task runner configuration with all recipes needed for the literate programming workflow. The justfile is the primary interface for building, testing, and maintaining the project.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 justfile exists in project root
- [ ] #2 Recipe 'tangle' runs notangle to generate src/main.rs from nw/00-main.nw
- [ ] #3 Recipe 'build' runs cargo build
- [ ] #4 Recipe 'test' runs cargo test
- [ ] #5 Recipe 'lint' runs cargo clippy
- [ ] #6 Recipe 'fmt' runs cargo fmt --check
- [ ] #7 Recipe 'fmt-fix' runs cargo fmt
- [ ] #8 Recipe 'weave' runs lualatex to produce PDF in doc/
- [ ] #9 Recipe 'clean' removes src/, doc/, and target/
- [ ] #10 Recipe 'check' runs tangle then cargo check
- [ ] #11 Recipe 'ci' runs tangle, build, test, and lint
- [ ] #12 Recipe 'all' runs tangle, build, and weave
- [ ] #13 Recipe 'watch' uses watchexec to re-tangle on .nw changes
- [ ] #14 Recipe 'view' opens generated PDF
- [ ] #15 Recipe 'e2e' placeholder exists
- [ ] #16 All recipes work within nix-shell
<!-- AC:END -->
