---
id: task-0109
title: Implement framed message reading
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:35'
updated_date: '2026-01-17 18:47'
labels:
  - phase-3
  - network
  - messages
  - framing
dependencies:
  - task-0073
  - task-0101
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Read length-prefixed messages from TCP stream. The protocol uses 4-byte big-endian length prefix for all messages after handshake. Must handle: partial reads (TCP can fragment), keep-alives (length=0), and oversized messages (protect against memory exhaustion).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 read_message(stream: &mut TcpStream) -> Result<Message, Error> reads one complete message
- [x] #2 First reads 4-byte length prefix
- [x] #3 Enforces MAX_MESSAGE_LENGTH (e.g., 2^17 = 131072 bytes for piece + overhead) to prevent memory exhaustion
- [x] #4 Returns Error::MessageTooLarge if length exceeds maximum
- [x] #5 Reads exactly length bytes for message body
- [x] #6 Returns Error::ConnectionClosed if stream closes mid-message
- [x] #7 Parses and returns Message using Message::from_bytes
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented read_message() async function in nw/07-client.nw.

Function:
  async fn read_message(stream) -> Result<Message, PeerError>

Implementation:
1. Reads 4-byte big-endian length prefix with read_exact
2. Returns Message::KeepAlive for length=0
3. Validates length <= MAX_MESSAGE_LENGTH (131072) before allocation
4. Returns PeerError::MessageTooLarge if exceeded
5. Reads message body with read_exact
6. Returns PeerError::ConnectionClosed on UnexpectedEof
7. Parses with Message::from_bytes

Constant: MAX_MESSAGE_LENGTH = 131072 (2^17)

Tests: max_message_length_is_reasonable, max_message_length_is_power_of_two, read_message_function_exists

All 664 tests pass.
<!-- SECTION:NOTES:END -->
