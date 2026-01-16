---
id: task-0002
title: Define peer protocol error types
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - phase-3
  - protocol
  - error-handling
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Before implementing any peer protocol code, we need well-defined error types that capture all failure modes. This enables fail-fast behavior with clear error messages, which is essential for debugging protocol issues. Error types should distinguish between handshake failures, invalid messages, timeouts, and connection drops.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Error enum includes: HandshakeFailed(String), InvalidMessage(String), UnexpectedMessageId(u8), MessageTooLarge(u32), BitfieldWrongLength { expected: usize, actual: usize }, Timeout, ConnectionClosed, IoError(io::Error)
- [ ] #2 All error variants implement Display with human-readable messages
- [ ] #3 Errors are defined in messages.rs (tangled from 04-messages.nw)
- [ ] #4 Unit tests verify Display output for each error variant
<!-- AC:END -->
