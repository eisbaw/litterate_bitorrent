---
id: task-0018
title: Implement Handshake serialization (to_bytes)
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - phase-3
  - protocol
  - handshake
  - serialization
dependencies:
  - task-0010
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handshake must be serialized to exactly 68 bytes for transmission. The wire format is: [pstrlen=19][pstr=BitTorrent protocol][reserved=00000000][info_hash][peer_id]. This is different from message framing (no length prefix). Serialization must be deterministic and match the protocol spec exactly.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Handshake::to_bytes() returns [u8; 68]
- [ ] #2 Byte layout: [0]=19, [1..20]=protocol string, [20..28]=zeros, [28..48]=info_hash, [48..68]=peer_id
- [ ] #3 Unit test verifies exact byte layout with known inputs
- [ ] #4 Round-trip test: to_bytes then from_bytes yields identical Handshake
<!-- AC:END -->
