---
id: task-0009
title: Create Cargo.toml with project dependencies
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - infrastructure
  - phase0
dependencies:
  - task-0001
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define the Rust project manifest with all dependencies needed for the BitTorrent client. This enables Cargo to resolve and build the project once tangled code exists.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cargo.toml exists in project root
- [ ] #2 Package name is 'literate-bittorrent'
- [ ] #3 Edition is 2021
- [ ] #4 tokio dependency with rt-multi-thread and net features
- [ ] #5 reqwest dependency for HTTP tracker communication
- [ ] #6 sha1 dependency for hashing
- [ ] #7 clap dependency with derive feature for CLI
- [ ] #8 bytes dependency for buffer manipulation
- [ ] #9 thiserror dependency for error types
- [ ] #10 tracing and tracing-subscriber dependencies for logging
- [ ] #11 cargo check passes (requires tangled src/ to exist first)
<!-- AC:END -->
