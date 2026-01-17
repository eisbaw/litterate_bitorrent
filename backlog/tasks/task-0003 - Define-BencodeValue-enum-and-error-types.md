---
id: task-0003
title: Define BencodeValue enum and error types
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 23:19'
labels:
  - bencode
  - foundation
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Foundation for all bencode operations. Defines the recursive enum representing bencoded data (Integer, Bytes, List, Dict with BTreeMap for canonical ordering) and error types for parsing failures. This is the data design that all subsequent bencode operations depend on.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 BencodeValue enum defined with Integer(i64), Bytes(Vec<u8>), List(Vec<BencodeValue>), Dict(BTreeMap<Vec<u8>, BencodeValue>) variants
- [x] #2 BencodeError type defined with InvalidFormat, UnexpectedEof, InvalidInteger, InvalidLength variants with descriptive messages
- [x] #3 Code compiles with cargo check
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/01-bencode.nw with LaTeX prose introducing bencode and Rust code chunks
2. Define BencodeValue enum with BTreeMap for Dict ordering
3. Define BencodeError using thiserror derive macros
4. Update nw/00-main.nw to add lib.rs chunk with bencode module inclusion
5. Update justfile tangle recipe to extract both main.rs and lib.rs and bencode.rs
6. Verify with just tangle && just check
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented BencodeValue enum and BencodeError types for the literate BitTorrent client.

Changes:
- Created nw/01-bencode.nw with LaTeX prose explaining bencode format and Rust code chunks
- BencodeValue enum uses BTreeMap for Dict variant to ensure canonical key ordering (required for info hash)
- BencodeError uses thiserror derive macros with four variants: InvalidFormat, UnexpectedEof, InvalidInteger, InvalidLength
- Updated nw/00-main.nw to add lib.rs chunk that includes pub mod bencode
- Updated justfile tangle recipe to extract main.rs, lib.rs, and bencode.rs

Verified with just tangle && just check - compiles successfully.
<!-- SECTION:NOTES:END -->
