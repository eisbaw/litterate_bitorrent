---
id: task-0003
title: Define BencodeValue enum and error types
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
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
- [ ] #1 BencodeValue enum defined with Integer(i64), Bytes(Vec<u8>), List(Vec<BencodeValue>), Dict(BTreeMap<Vec<u8>, BencodeValue>) variants
- [ ] #2 BencodeError type defined with InvalidFormat, UnexpectedEof, InvalidInteger, InvalidLength variants with descriptive messages
- [ ] #3 Code compiles with cargo check
<!-- AC:END -->
