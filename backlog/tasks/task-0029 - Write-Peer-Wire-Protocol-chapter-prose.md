---
id: task-0029
title: Write Peer Wire Protocol chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 00:10'
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
Added comprehensive prose documentation to nw/04-messages.nw:

- Added Protocol Overview section with two-phase explanation (handshake vs message exchange)
- Added Message Framing subsection with ASCII diagram showing length-prefixed format
- Added Choking/Interested State Machine section with 4-state diagram showing when data can flow
- Enhanced Handshake Wire Format with byte offsets and field breakdown
- Added Detailed Message Wire Formats section with ASCII diagrams for all 9 message types
- Added Message Types Summary table for quick reference

Key prose additions:
- Clear explanation that data can only flow when: requester is interested AND sender has unchoked
- Bitfield encoding example showing spare bit handling
- Request/Piece/Cancel format with typical values

Verified: just tangle and just weave both succeed, all 134 messages module tests pass.
<!-- SECTION:NOTES:END -->
