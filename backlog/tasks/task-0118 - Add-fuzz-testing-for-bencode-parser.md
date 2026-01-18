---
id: task-0118
title: Add fuzz testing for bencode parser
status: In Progress
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 09:48'
labels:
  - testing
  - security
  - quality
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement cargo-fuzz targets for the bencode decoder to catch edge cases and malformed input handling. The bencode module is critical for parsing untrusted .torrent files and peer messages. Add fuzz targets for decode(), encode(), and round-trip testing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Add fuzz directory with Cargo.toml for cargo-fuzz
- [ ] #2 Create fuzz target for decode() with arbitrary bytes
- [ ] #3 Create fuzz target for encode() then decode() round-trip
- [ ] #4 Document how to run fuzzing in README or justfile
- [ ] #5 Run fuzzer for at least 10 minutes without crashes
<!-- AC:END -->
