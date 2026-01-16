---
id: task-0010
title: Implement Handshake struct and constants
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - phase-3
  - protocol
  - handshake
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The handshake is a fixed 68-byte structure sent at connection start, distinct from the length-prefixed message framing used afterward. It consists of: pstrlen (1 byte, value 19), pstr (19 bytes, 'BitTorrent protocol'), reserved (8 bytes, zeros), info_hash (20 bytes), peer_id (20 bytes). This is the foundation for peer identification and torrent validation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Handshake struct with fields: info_hash: [u8; 20], peer_id: [u8; 20]
- [ ] #2 Constant PROTOCOL_STRING = b"BitTorrent protocol" (19 bytes)
- [ ] #3 Constant HANDSHAKE_LENGTH = 68
- [ ] #4 Reserved bytes are 8 zeros (no extension bits for now)
- [ ] #5 Handshake::new(info_hash, peer_id) constructor validates inputs
<!-- AC:END -->
