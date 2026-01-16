---
id: task-0106
title: Implement async handshake exchange
status: To Do
assignee: []
created_date: '2026-01-16 21:35'
labels:
  - phase-3
  - network
  - handshake
dependencies:
  - task-0018
  - task-0036
  - task-0101
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After TCP connection, perform the handshake exchange: send our handshake (68 bytes), receive peer's handshake (68 bytes), validate it. This must be done with timeout since peers may connect but never respond. The handshake validates both peers are talking about the same torrent.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 perform_handshake(stream: &mut TcpStream, our_handshake: &Handshake, expected_info_hash: &[u8; 20]) -> Result<Handshake, Error>
- [ ] #2 Sends our handshake bytes using AsyncWriteExt::write_all
- [ ] #3 Reads exactly 68 bytes using AsyncReadExt::read_exact
- [ ] #4 Entire handshake exchange completes within 10 seconds (HANDSHAKE_TIMEOUT_SECS)
- [ ] #5 Returns Error::Timeout if peer does not respond in time
- [ ] #6 Validates received handshake: correct protocol string, matching info_hash
- [ ] #7 Returns parsed peer Handshake on success (contains peer_id)
<!-- AC:END -->
