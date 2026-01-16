---
id: task-0073
title: Implement message parsing (from_bytes)
status: To Do
assignee: []
created_date: '2026-01-16 21:30'
labels:
  - phase-3
  - protocol
  - messages
  - parsing
dependencies:
  - task-0052
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse messages from bytes received from peers. First read 4-byte length prefix, then read that many bytes for the message body. Handle keep-alive (length=0) specially. Validate message ID and payload length for each type. Return clear errors for malformed messages.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Message::from_bytes(bytes: &[u8]) -> Result<Message, Error> parses a complete message (including length prefix)
- [ ] #2 Returns KeepAlive when length=0
- [ ] #3 Returns error for unknown message ID
- [ ] #4 Validates Have payload is exactly 4 bytes
- [ ] #5 Validates Request/Cancel payload is exactly 12 bytes
- [ ] #6 Validates Piece payload is at least 8 bytes (index + begin)
- [ ] #7 Returns InvalidMessage error with context for malformed input
- [ ] #8 Unit tests cover valid messages and each validation failure case
<!-- AC:END -->
