---
id: task-0052
title: Define Message enum for peer wire protocol
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 09:22'
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
- [x] #1 Message enum with variants: KeepAlive, Choke, Unchoke, Interested, NotInterested, Have(u32), Bitfield(Vec<u8>), Request { index: u32, begin: u32, length: u32 }, Piece { index: u32, begin: u32, data: Vec<u8> }, Cancel { index: u32, begin: u32, length: u32 }
- [x] #2 Message ID constants: CHOKE=0, UNCHOKE=1, INTERESTED=2, NOT_INTERESTED=3, HAVE=4, BITFIELD=5, REQUEST=6, PIECE=7, CANCEL=8
- [x] #3 Message::id() returns Option<u8> (None for KeepAlive)
- [x] #4 Standard block size constant: BLOCK_SIZE = 16384 (2^14)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Study existing 04-messages.nw structure and code conventions
2. Add message ID constants (CHOKE=0 through CANCEL=8)
3. Add BLOCK_SIZE constant (16384)
4. Define Message enum with all 10 variants per BEP 3
5. Implement Message::id() method returning Option<u8>
6. Write literate prose explaining each message type and its purpose
7. Add comprehensive tests for Message enum and id() method
8. Run just check && just test to verify implementation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented Message enum for peer wire protocol per BEP 3:

- Added message ID constants: MSG_CHOKE=0, MSG_UNCHOKE=1, MSG_INTERESTED=2, MSG_NOT_INTERESTED=3, MSG_HAVE=4, MSG_BITFIELD=5, MSG_REQUEST=6, MSG_PIECE=7, MSG_CANCEL=8
- Added BLOCK_SIZE constant (16384 = 2^14 bytes)
- Defined Message enum with 10 variants: KeepAlive, Choke, Unchoke, Interested, NotInterested, Have(u32), Bitfield(Vec<u8>), Request{index,begin,length}, Piece{index,begin,data}, Cancel{index,begin,length}
- Implemented Message::id() method returning Option<u8> (None for KeepAlive)
- Added comprehensive tests (20+ tests) covering all message types, constants, and edge cases
- Wrote literate prose explaining protocol semantics for each message type

All 284 tests pass. Modified file: nw/04-messages.nw
<!-- SECTION:NOTES:END -->
