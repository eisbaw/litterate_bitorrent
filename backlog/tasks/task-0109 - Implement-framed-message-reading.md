---
id: task-0109
title: Implement framed message reading
status: To Do
assignee: []
created_date: '2026-01-16 21:35'
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
- [ ] #1 read_message(stream: &mut TcpStream) -> Result<Message, Error> reads one complete message
- [ ] #2 First reads 4-byte length prefix
- [ ] #3 Enforces MAX_MESSAGE_LENGTH (e.g., 2^17 = 131072 bytes for piece + overhead) to prevent memory exhaustion
- [ ] #4 Returns Error::MessageTooLarge if length exceeds maximum
- [ ] #5 Reads exactly length bytes for message body
- [ ] #6 Returns Error::ConnectionClosed if stream closes mid-message
- [ ] #7 Parses and returns Message using Message::from_bytes
<!-- AC:END -->
