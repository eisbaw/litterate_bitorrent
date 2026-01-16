---
id: task-0013
title: Generate 20-byte peer_id with client identifier
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - tracker
  - identity
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each BitTorrent client instance needs a unique 20-byte peer_id sent to trackers and peers. The peer_id identifies this specific client session. Convention is to start with a client identifier (e.g., -LT0001- for Literate Torrent v0.0.01) followed by random bytes to ensure uniqueness across sessions. The format helps debugging and client identification in swarms.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Function generates 20-byte peer_id as [u8; 20]
- [ ] #2 First 8 bytes follow Azureus-style convention: -LT0001- (or similar client prefix)
- [ ] #3 Remaining 12 bytes are random (use rand crate or getrandom)
- [ ] #4 peer_id is generated once per client session and reused
- [ ] #5 Unit test: generated peer_id is exactly 20 bytes
- [ ] #6 Unit test: peer_id prefix matches expected client identifier
<!-- AC:END -->
