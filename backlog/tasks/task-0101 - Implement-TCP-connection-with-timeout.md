---
id: task-0101
title: Implement TCP connection with timeout
status: To Do
assignee: []
created_date: '2026-01-16 21:34'
labels:
  - phase-3
  - network
  - tcp
dependencies:
  - task-0002
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Establish TCP connection to a peer with 10-second timeout. This is the foundation for all peer communication. Use tokio for async operation. The connection timeout prevents hanging on unresponsive peers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 connect(addr: SocketAddr) -> Result<TcpStream, Error> with 10s timeout
- [ ] #2 Returns Error::Timeout if connection not established within 10 seconds
- [ ] #3 Returns Error::IoError for other connection failures (refused, unreachable)
- [ ] #4 Uses tokio::net::TcpStream with tokio::time::timeout
- [ ] #5 Connection timeout is configurable via constant CONNECT_TIMEOUT_SECS = 10
<!-- AC:END -->
