---
id: task-0077
title: Implement compact peer list parsing (BEP-23)
status: To Do
assignee: []
created_date: '2026-01-16 21:31'
labels:
  - tracker
  - parsing
  - bep23
dependencies:
  - task-0069
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse the compact peer format where each peer is exactly 6 bytes: 4 bytes for IPv4 address (big-endian) and 2 bytes for port (big-endian). This is the preferred format (BEP-23) and most trackers support it when compact=1 is requested. The parser must handle arbitrary numbers of peers and validate the byte length is divisible by 6.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Function takes &[u8], returns Result<Vec<SocketAddr>, TrackerError>
- [ ] #2 Each 6-byte chunk parsed as 4-byte IPv4 + 2-byte port (big-endian)
- [ ] #3 Returns error if byte length not divisible by 6
- [ ] #4 Empty input returns empty Vec (valid: no peers available)
- [ ] #5 Unit test: parse 12 bytes into 2 peers with correct IP:port
- [ ] #6 Unit test: parse 0 bytes returns empty vec
- [ ] #7 Unit test: parse 7 bytes returns error (not divisible by 6)
- [ ] #8 Unit test: verify endianness with known test vector
<!-- AC:END -->
