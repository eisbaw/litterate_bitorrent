---
id: task-0064
title: Implement message serialization (to_bytes)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:30'
updated_date: '2026-01-17 09:28'
labels:
  - phase-3
  - protocol
  - messages
  - serialization
dependencies:
  - task-0052
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Messages must be serialized to bytes for transmission. The wire format is: 4-byte big-endian length prefix, followed by message ID (1 byte, except keep-alive), followed by payload. Each message type has specific payload encoding. Serialization must produce correct wire format that peers will accept.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Message::to_bytes() returns Vec<u8> with length prefix included
- [x] #2 KeepAlive serializes to [0,0,0,0] (length=0, no ID)
- [x] #3 Choke/Unchoke/Interested/NotInterested serialize to [0,0,0,1,id] (length=1)
- [x] #4 Have serializes to [0,0,0,5,4,piece_index_be] (length=5)
- [x] #5 Bitfield serializes to [length_be,5,bitfield_bytes] where length=1+bitfield.len()
- [x] #6 Request/Cancel serialize to [0,0,0,13,id,index_be,begin_be,length_be] (length=13)
- [x] #7 Piece serializes to [length_be,7,index_be,begin_be,data] where length=9+data.len()
- [x] #8 Unit tests verify exact byte output for each message type
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add to_bytes method to Message enum in 04-messages.nw
2. Write literate prose explaining serialization format per BEP 3
3. Implement serialization for each message type:
   - KeepAlive: [0,0,0,0]
   - Choke/Unchoke/Interested/NotInterested: [0,0,0,1,id]
   - Have: [0,0,0,5,4,piece_index_be]
   - Bitfield: [length_be,5,bitfield_bytes]
   - Request/Cancel: [0,0,0,13,id,index,begin,length]
   - Piece: [length_be,7,index,begin,data]
4. Add comprehensive unit tests for all message types
5. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added Message::to_bytes() method implementing BEP 3 wire format serialization:

- KeepAlive: [0,0,0,0] (length=0, no ID)
- Choke/Unchoke/Interested/NotInterested: [0,0,0,1,id] (length=1)
- Have: [0,0,0,5,4,piece_index_be] (length=5)
- Bitfield: [length_be,5,bitfield_bytes] (length=1+n)
- Request/Cancel: [0,0,0,13,id,index_be,begin_be,length_be] (length=13)
- Piece: [length_be,7,index_be,begin_be,data] (length=9+n)

All integers use big-endian encoding. Implementation uses Vec::with_capacity for efficient allocation.

Added comprehensive test coverage with 20+ new tests verifying exact byte output for all message types.
<!-- SECTION:NOTES:END -->
