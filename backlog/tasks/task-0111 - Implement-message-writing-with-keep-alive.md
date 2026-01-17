---
id: task-0111
title: Implement message writing with keep-alive
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:36'
updated_date: '2026-01-17 18:57'
labels:
  - phase-3
  - network
  - messages
dependencies:
  - task-0064
  - task-0101
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Write messages to TCP stream with proper framing. Track last message send time to automatically send keep-alives if idle for too long (120 seconds). Keep-alives prevent the connection from being closed by the peer.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 write_message(stream: &mut TcpStream, msg: &Message) -> Result<(), Error> writes serialized message
- [x] #2 Uses AsyncWriteExt::write_all for atomic write
- [x] #3 Tracks last_send_time for keep-alive scheduling
- [x] #4 KEEP_ALIVE_INTERVAL = 120 seconds
- [x] #5 send_keep_alive_if_needed(stream, last_send_time) -> Result<(), Error> sends keep-alive if interval exceeded
- [x] #6 Returns Error::IoError on write failure
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add new subsection "Framed Message Writing" after "Framed Message Reading Tests" section
2. Add KEEP_ALIVE_INTERVAL_SECS constant
3. Add write_message function using Message::to_bytes and write_all
4. Add send_keep_alive_if_needed function with Instant tracking
5. Add std::time::Instant import
6. Add code chunk and test chunk references to module structure
7. Create tests for the new functions
8. Run just check && cargo test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented message writing functions in nw/07-client.nw.

Functions:
- write_message(stream, msg) - writes serialized message with write_all
- send_keep_alive_if_needed(stream, last_send_time) - sends keep-alive if interval exceeded

Constant: KEEP_ALIVE_INTERVAL_SECS = 120 (per BEP 3)

Design:
- write_all ensures atomic writes (completes or fails)
- send_keep_alive_if_needed returns updated Instant for tracking
- I/O errors propagate as PeerError::IoError

Tests: keep_alive_interval_is_120_seconds, write_message_function_exists, send_keep_alive_if_needed_function_exists

All 667 tests pass.
<!-- SECTION:NOTES:END -->
