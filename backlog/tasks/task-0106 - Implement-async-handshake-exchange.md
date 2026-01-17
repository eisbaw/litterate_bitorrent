---
id: task-0106
title: Implement async handshake exchange
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:35'
updated_date: '2026-01-17 18:36'
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
- [x] #1 perform_handshake(stream: &mut TcpStream, our_handshake: &Handshake, expected_info_hash: &[u8; 20]) -> Result<Handshake, Error>
- [x] #2 Sends our handshake bytes using AsyncWriteExt::write_all
- [x] #3 Reads exactly 68 bytes using AsyncReadExt::read_exact
- [x] #4 Entire handshake exchange completes within 10 seconds (HANDSHAKE_TIMEOUT_SECS)
- [x] #5 Returns Error::Timeout if peer does not respond in time
- [x] #6 Validates received handshake: correct protocol string, matching info_hash
- [x] #7 Returns parsed peer Handshake on success (contains peer_id)
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented perform_handshake() async function in nw/07-client.nw.

Function signature:
  async fn perform_handshake(stream, our_handshake, expected_info_hash) -> Result<Handshake, PeerError>

Implementation:
1. Sends 68-byte handshake with AsyncWriteExt::write_all
2. Reads 68-byte response with AsyncReadExt::read_exact
3. Parses with Handshake::from_bytes (validates protocol string)
4. Validates info_hash match with validate_info_hash()
5. Entire exchange wrapped in 10-second timeout

Constant: HANDSHAKE_TIMEOUT_SECS = 10

Tests: handshake_timeout_constant_is_10_seconds, perform_handshake_function_exists

All 661 tests pass.
<!-- SECTION:NOTES:END -->
