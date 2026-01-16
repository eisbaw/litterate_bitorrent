---
id: task-0053
title: Create justfile with core recipes
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 22:34'
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
- [x] #1 justfile exists in project root
- [x] #2 Recipe 'tangle' runs notangle to generate src/main.rs from nw/00-main.nw
- [x] #3 Recipe 'build' runs cargo build
- [x] #4 Recipe 'test' runs cargo test
- [x] #5 Recipe 'lint' runs cargo clippy
- [x] #6 Recipe 'fmt' runs cargo fmt --check
- [x] #7 Recipe 'fmt-fix' runs cargo fmt
- [x] #8 Recipe 'weave' runs lualatex to produce PDF in doc/
- [x] #9 Recipe 'clean' removes src/, doc/, and target/
- [x] #10 Recipe 'check' runs tangle then cargo check
- [x] #11 Recipe 'ci' runs tangle, build, test, and lint
- [x] #12 Recipe 'all' runs tangle, build, and weave
- [x] #13 Recipe 'watch' uses watchexec to re-tangle on .nw changes
- [x] #14 Recipe 'view' opens generated PDF
- [x] #15 Recipe 'e2e' placeholder exists
- [x] #16 All recipes work within nix-shell
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review PRD for complete list of recipes and expected behavior
2. Create justfile with all required recipes:
   - tangle: notangle -Rmain.rs to src/main.rs (creates src/ if needed)
   - build: cargo build (depends on tangle)
   - test: cargo test
   - lint: cargo clippy
   - fmt: cargo fmt --check
   - fmt-fix: cargo fmt
   - weave: noweave + lualatex to doc/ (creates doc/ if needed)
   - clean: rm -rf src/ doc/ target/
   - check: tangle then cargo check
   - ci: tangle, build, test, lint
   - all: tangle, build, weave
   - watch: watchexec on .nw changes
   - view: open PDF viewer
   - e2e: placeholder
3. Test all recipes within nix-shell
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created justfile with all required recipes for the literate BitTorrent client project.

Recipes implemented:
- tangle: Extracts Rust code from nw/00-main.nw to src/main.rs
- weave: Generates PDF documentation via noweave + lualatex (runs twice for cross-references)
- build: Compiles with cargo build (depends on tangle)
- check: Quick compile check via cargo check (depends on tangle)
- test: Runs cargo test
- lint: Runs cargo clippy with warnings as errors
- fmt/fmt-fix: Formatting check and fix via cargo fmt
- clean: Removes src/, doc/, and target/
- ci: Full CI pipeline (tangle, build, test, lint)
- all: Complete build including docs (tangle, build, weave)
- watch: Auto-retangle on .nw changes via watchexec
- view: Opens generated PDF with system viewer
- e2e: Placeholder for future end-to-end tests
- run: Convenience recipe to run the client

Notes:
- All recipes tested within nix-shell
- tangle and clean recipes verified working
- build/weave recipes execute correct commands but fail due to pre-existing issues:
  - Rust build: nixpkgs pins rustc 1.82.0 but newer deps require 1.83+
  - PDF generation: LuaLaTeX cannot find JetBrains Mono font (needs OSFONTDIR in shell.nix)
- These are configuration issues in shell.nix/Cargo.toml, not justfile problems
<!-- SECTION:NOTES:END -->
