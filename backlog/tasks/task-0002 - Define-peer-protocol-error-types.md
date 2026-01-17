---
id: task-0002
title: Define peer protocol error types
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 09:01'
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
- [x] #1 Error enum includes: HandshakeFailed(String), InvalidMessage(String), UnexpectedMessageId(u8), MessageTooLarge(u32), BitfieldWrongLength { expected: usize, actual: usize }, Timeout, ConnectionClosed, IoError(io::Error)
- [x] #2 All error variants implement Display with human-readable messages
- [x] #3 Errors are defined in messages.rs (tangled from 04-messages.nw)
- [x] #4 Unit tests verify Display output for each error variant
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/04-messages.nw with LaTeX preamble and intro prose explaining peer protocol errors
2. Define PeerError enum with all required variants using thiserror
3. Write literate prose explaining each error variant
4. Add unit tests for Display output of each variant
5. Update nw/00-main.nw to add `pub mod messages;`
6. Update justfile to tangle messages.rs
7. Verify with `nix-shell --run "just check && just test"`
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented PeerError enum in nw/04-messages.nw with all 8 required variants:
- HandshakeFailed(String) - for initial 68-byte handshake failures
- InvalidMessage(String) - for malformed wire protocol messages
- UnexpectedMessageId(u8) - for unknown message type IDs
- MessageTooLarge(u32) - safety limit for oversized messages
- BitfieldWrongLength { expected, actual } - struct variant for diagnostic details
- Timeout - for operations exceeding protocol timeouts
- ConnectionClosed - clean EOF from peer
- IoError(#[from] io::Error) - wraps underlying I/O errors with automatic conversion

Used thiserror derive macro for Display and Error trait implementations. Each variant has comprehensive rustdoc documenting the error cause and recommended recovery strategy.

Added 12 unit tests covering:
- Display output verification for all 8 variants
- std::error::Error trait implementation
- Debug trait implementation
- Send + Sync trait bounds
- Automatic io::Error conversion via #[from]

Files modified:
- nw/04-messages.nw (new) - literate source with LaTeX prose
- nw/00-main.nw - added pub mod messages to lib.rs
- justfile - added tangle recipe for messages.rs
<!-- SECTION:NOTES:END -->
