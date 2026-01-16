---
id: task-0111
title: Implement message writing with keep-alive
status: To Do
assignee: []
created_date: '2026-01-16 21:36'
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
- [ ] #1 write_message(stream: &mut TcpStream, msg: &Message) -> Result<(), Error> writes serialized message
- [ ] #2 Uses AsyncWriteExt::write_all for atomic write
- [ ] #3 Tracks last_send_time for keep-alive scheduling
- [ ] #4 KEEP_ALIVE_INTERVAL = 120 seconds
- [ ] #5 send_keep_alive_if_needed(stream, last_send_time) -> Result<(), Error> sends keep-alive if interval exceeded
- [ ] #6 Returns Error::IoError on write failure
<!-- AC:END -->
