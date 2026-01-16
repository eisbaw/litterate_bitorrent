---
id: task-0052
title: Define Message enum for peer wire protocol
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
labels:
  - phase-3
  - protocol
  - messages
dependencies:
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The peer wire protocol defines 9 message types with IDs 0-8, plus keep-alive (length=0, no ID). Each message has specific payload structure. The Message enum provides type-safe representation of all protocol messages, enabling pattern matching for message handling and ensuring all message types are explicitly handled.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Message enum with variants: KeepAlive, Choke, Unchoke, Interested, NotInterested, Have(u32), Bitfield(Vec<u8>), Request { index: u32, begin: u32, length: u32 }, Piece { index: u32, begin: u32, data: Vec<u8> }, Cancel { index: u32, begin: u32, length: u32 }
- [ ] #2 Message ID constants: CHOKE=0, UNCHOKE=1, INTERESTED=2, NOT_INTERESTED=3, HAVE=4, BITFIELD=5, REQUEST=6, PIECE=7, CANCEL=8
- [ ] #3 Message::id() returns Option<u8> (None for KeepAlive)
- [ ] #4 Standard block size constant: BLOCK_SIZE = 16384 (2^14)
<!-- AC:END -->
