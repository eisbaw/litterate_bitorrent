---
id: task-0064
title: Implement message serialization (to_bytes)
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
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
- [ ] #1 Message::to_bytes() returns Vec<u8> with length prefix included
- [ ] #2 KeepAlive serializes to [0,0,0,0] (length=0, no ID)
- [ ] #3 Choke/Unchoke/Interested/NotInterested serialize to [0,0,0,1,id] (length=1)
- [ ] #4 Have serializes to [0,0,0,5,4,piece_index_be] (length=5)
- [ ] #5 Bitfield serializes to [length_be,5,bitfield_bytes] where length=1+bitfield.len()
- [ ] #6 Request/Cancel serialize to [0,0,0,13,id,index_be,begin_be,length_be] (length=13)
- [ ] #7 Piece serializes to [length_be,7,index_be,begin_be,data] where length=9+data.len()
- [ ] #8 Unit tests verify exact byte output for each message type
<!-- AC:END -->
