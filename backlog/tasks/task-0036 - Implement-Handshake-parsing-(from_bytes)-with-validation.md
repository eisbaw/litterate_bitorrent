---
id: task-0036
title: Implement Handshake parsing (from_bytes) with validation
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels:
  - phase-3
  - protocol
  - handshake
  - parsing
dependencies:
  - task-0010
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse handshake from exactly 68 bytes received from a peer. Must validate: pstrlen is 19, pstr matches 'BitTorrent protocol' exactly, info_hash matches expected (critical security check to ensure peer is on same torrent). Invalid handshakes must fail fast with clear error messages.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Handshake::from_bytes(bytes: &[u8]) -> Result<Handshake, Error>
- [ ] #2 Returns error if input length \!= 68
- [ ] #3 Returns error if pstrlen \!= 19
- [ ] #4 Returns error if pstr \!= b"BitTorrent protocol"
- [ ] #5 Handshake::validate_info_hash(expected: &[u8; 20]) -> Result<(), Error>
- [ ] #6 Unit tests cover: valid handshake, wrong pstrlen, wrong pstr, wrong length
<!-- AC:END -->
