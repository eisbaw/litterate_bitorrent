---
id: task-0101
title: Implement TCP connection with timeout
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:34'
updated_date: '2026-01-17 18:28'
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
- [x] #1 connect(addr: SocketAddr) -> Result<TcpStream, Error> with 10s timeout
- [x] #2 Returns Error::Timeout if connection not established within 10 seconds
- [x] #3 Returns Error::IoError for other connection failures (refused, unreachable)
- [x] #4 Uses tokio::net::TcpStream with tokio::time::timeout
- [x] #5 Connection timeout is configurable via constant CONNECT_TIMEOUT_SECS = 10
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add connect_with_timeout function to nw/07-client.nw in the peer connection manager chunk
2. Add literate prose explaining why connection timeouts are needed
3. Add tests for connect_with_timeout (refused connection, timeout constant verification)
4. Use existing PeerError::Timeout and PeerError::IoError variants (no breaking changes)
5. Run just check && cargo test connect_with_timeout to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented connect_with_timeout() async function in nw/07-client.nw.

Function signature:
  async fn connect_with_timeout(addr: SocketAddr) -> Result<TcpStream, PeerError>

Uses tokio::time::timeout to wrap TcpStream::connect with CONNECT_TIMEOUT_SECS (10).
Returns PeerError::Timeout on timeout, PeerError::IoError for connection refused/unreachable.

Tests: connect_with_timeout_returns_error_for_refused, connect_with_timeout_uses_correct_timeout

All 659 tests pass.
<!-- SECTION:NOTES:END -->
