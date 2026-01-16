---
id: task-0001
title: Create shell.nix with development environment
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
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
- [ ] #1 shell.nix exists and uses pinned nixpkgs
- [ ] #2 Rust toolchain (stable) is available in shell
- [ ] #3 noweb tool is available in shell
- [ ] #4 TeX Live with scheme-medium is available
- [ ] #5 minted and Pygments are available for code highlighting
- [ ] #6 just task runner is available
- [ ] #7 watchexec is available for file watching
- [ ] #8 Running 'nix-shell --run "rustc --version && notangle --version && lualatex --version && just --version"' succeeds
<!-- AC:END -->
