---
id: task-0029
title: Write Peer Wire Protocol chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The peer wire protocol is the heart of BitTorrent - how peers communicate. This chapter must clearly separate the handshake (fixed format, not length-prefixed) from subsequent messages (length-prefixed). The 4-bit choking/interested state machine needs careful explanation with a state diagram. Each message type should have byte-level format documentation and usage context. The reader should understand how data flow is controlled through choking and interest signaling.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Handshake format is documented byte-by-byte with pstrlen, pstr, reserved, info_hash, peer_id
- [ ] #2 Message framing (4-byte length prefix) is explained with diagram
- [ ] #3 All message types documented: keep-alive, choke/unchoke, interested/not interested, have, bitfield, request, piece, cancel
- [ ] #4 Choking/interested state machine has diagram and explanation of data flow conditions
- [ ] #5 Reader understands when data can flow: not choked AND interested
<!-- AC:END -->
