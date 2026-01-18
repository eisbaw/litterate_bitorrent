---
id: task-0029
title: Write Peer Wire Protocol chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 00:14'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The peer wire protocol is the heart of BitTorrent - how peers communicate. This chapter must clearly separate the handshake (fixed format, not length-prefixed) from subsequent messages (length-prefixed). The 4-bit choking/interested state machine needs careful explanation with a state diagram. Each message type should have byte-level format documentation and usage context. The reader should understand how data flow is controlled through choking and interest signaling.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Handshake format is documented byte-by-byte with pstrlen, pstr, reserved, info_hash, peer_id
- [x] #2 Message framing (4-byte length prefix) is explained with diagram
- [x] #3 All message types documented: keep-alive, choke/unchoke, interested/not interested, have, bitfield, request, piece, cancel
- [x] #4 Choking/interested state machine has diagram and explanation of data flow conditions
- [x] #5 Reader understands when data can flow: not choked AND interested
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add comprehensive protocol overview section at the beginning after existing intro
2. Enhance handshake format documentation with clearer byte-by-byte diagram
3. Add dedicated message framing section with ASCII diagram
4. Add comprehensive choking/interested state machine section with state diagram
5. Add data flow conditions explanation
6. Run just tangle and just weave to verify
7. Mark acceptance criteria complete
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added comprehensive Peer Wire Protocol chapter prose:
- Protocol overview with handshake vs message phases
- Message framing diagram with length prefix
- Choking/interested state machine diagram
- Message types summary table
- Wire format diagrams for all 9 message types
<!-- SECTION:NOTES:END -->
