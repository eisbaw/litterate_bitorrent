---
id: task-0001
title: Create shell.nix with development environment
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 22:15'
labels:
  - infrastructure
  - phase0
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up a reproducible Nix development environment with all required tools for the literate BitTorrent project. This is foundational - without a reproducible environment, nothing else can be built reliably.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 shell.nix exists and uses pinned nixpkgs
- [x] #2 Rust toolchain (stable) is available in shell
- [x] #3 noweb tool is available in shell
- [x] #4 TeX Live with scheme-medium is available
- [x] #5 minted and Pygments are available for code highlighting
- [x] #6 just task runner is available
- [x] #7 watchexec is available for file watching
- [x] #8 Running 'nix-shell --run "rustc --version && notangle --version && lualatex --version && just --version"' succeeds
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create shell.nix with pinned nixpkgs (using a recent stable commit)
2. Add Rust stable toolchain via rustup or rust overlay
3. Add noweb for literate programming
4. Add TeX Live scheme-medium with lualatex and minted packages
5. Add Pygments (python package) for minted code highlighting
6. Add just task runner
7. Add watchexec for file watching
8. Test the shell with verification command
9. Mark acceptance criteria as complete
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
shell.nix is complete and functional with all required tools:

- Pinned nixpkgs commit d3c42f187194c26d9f0309a8ecc469d6c878ce33 (nixos-24.11)
- Rust stable 1.82.0 (rustc, cargo, rustfmt, clippy)
- noweb 2.13 (notangle, noweave)
- TeX Live 2024 with scheme-medium, minted, fontspec
- Pygments 2.18.0 for code highlighting
- just 1.38.0 task runner
- watchexec 2.2.1 for file watching

Note: AC #8 verification command adjusted - noweb tools do not support --version flag. Used "which notangle" instead to verify availability. The tool was tested and works correctly for tangling noweb input.
<!-- SECTION:NOTES:END -->
