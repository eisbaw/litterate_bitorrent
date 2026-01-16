---
id: task-0009
title: Create Cargo.toml with project dependencies
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 22:20'
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
- [x] #1 Cargo.toml exists in project root
- [x] #2 Package name is 'literate-bittorrent'
- [x] #3 Edition is 2021
- [x] #4 tokio dependency with rt-multi-thread and net features
- [x] #5 reqwest dependency for HTTP tracker communication
- [x] #6 sha1 dependency for hashing
- [x] #7 clap dependency with derive feature for CLI
- [x] #8 bytes dependency for buffer manipulation
- [x] #9 thiserror dependency for error types
- [x] #10 tracing and tracing-subscriber dependencies for logging
- [ ] #11 cargo check passes (requires tangled src/ to exist first)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review PRD for exact dependency requirements
2. Create Cargo.toml with:
   - Package metadata (name, version, edition)
   - Runtime deps: tokio, reqwest, sha1, clap, bytes, thiserror, tracing, tracing-subscriber
   - Dev deps: proptest for property testing
3. Verify file was created correctly
4. Note: Skip cargo check verification per user instructions (no tangled src/ yet)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created Cargo.toml with all required dependencies:

- tokio 1.x with rt-multi-thread, net, io-util, time, sync, macros, fs features
- reqwest 0.12 with rustls-tls (avoiding native OpenSSL for portability)
- sha1 0.10 for hashing
- clap 4.x with derive feature
- bytes 1.x for buffer manipulation
- thiserror 2.x for error types
- tracing 0.1 and tracing-subscriber 0.3 with env-filter
- proptest 1.x as dev-dependency for property testing

Note: AC #11 (cargo check) skipped as instructed - requires tangled src/ which does not exist yet.
<!-- SECTION:NOTES:END -->
