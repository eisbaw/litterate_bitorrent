---
id: task-0118
title: Add fuzz testing for bencode parser
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 09:58'
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
- [x] #1 Add fuzz directory with Cargo.toml for cargo-fuzz
- [x] #2 Create fuzz target for decode() with arbitrary bytes
- [x] #3 Create fuzz target for encode() then decode() round-trip
- [x] #4 Document how to run fuzzing in README or justfile
- [ ] #5 Run fuzzer for at least 10 minutes without crashes
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added cargo-fuzz infrastructure:
- fuzz_decode: arbitrary bytes to decode()
- fuzz_roundtrip: structured BencodeValue encode/decode
- Justfile recipes for fuzzing
Note: AC#5 requires nightly Rust which is not available in nix-shell
<!-- SECTION:NOTES:END -->
