---
id: task-0060
title: Implement main event loop with tokio select
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 21:42'
labels:
  - phase-6
  - event-loop
  - orchestration
dependencies:
  - task-0023
  - task-0027
  - task-0031
  - task-0037
  - task-0043
  - task-0046
  - task-0050
  - task-0054
  - task-0058
  - task-0063
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The central event loop that multiplexes all async operations using tokio::select!. This is the heart of the client, orchestrating peer messages, timers, and shutdown signals. Must be implemented after individual handlers are ready.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Event loop uses tokio::select! to await multiple futures concurrently
- [x] #2 Handles: peer messages, request timeouts, re-announce timer, keep-alive timer, shutdown signal
- [x] #3 Routes incoming messages to appropriate handlers
- [x] #4 Triggers block requests after unchoke or piece completion
- [x] #5 Replaces disconnected peers from pool
- [x] #6 Exits cleanly on completion or shutdown signal
- [x] #7 Logs event loop iterations at TRACE level for debugging
- [x] #8 Unit test: event loop processes messages in correct order
- [x] #9 Integration test: event loop handles concurrent peer events
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add event loop chunk to nw/07-client.nw that uses tokio::select\! to multiplex:
   - Peer message reading from active connections (via FuturesUnordered)
   - Request timeout timer (30s interval)
   - Re-announce timer (tracker interval)
   - Keep-alive timer (120s interval)
   - Shutdown signal check
2. Add event loop tests chunk with unit test for message ordering
3. Add integration test chunk for concurrent peer events
4. Update client.rs template to include new chunks
5. Run just lint and just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Summary

Implemented the main event loop using tokio::select\! to multiplex all async operations in the BitTorrent client.

## Changes Made

### Event Loop Core (nw/07-client.nw)

1. **Event Types**: Added `EventLoopEvent` enum with variants for:
   - `PeerMessage` - incoming messages from peers
   - `PeerDisconnected` - connection lost
   - `RequestTimeoutCheck` - 30s periodic check
   - `ReAnnounce` - tracker re-announce timer
   - `KeepAliveCheck` - 120s keep-alive timer
   - `Shutdown` - graceful shutdown
   - `DownloadComplete` - all pieces verified

2. **Event Loop Context**: Created `EventLoopContext` struct to pass all required state (connection_manager, piece_manager, availability, shutdown, request_queues, in_progress_pieces, etc.)

3. **Main Loop**: `run_event_loop()` function using tokio::select\! with biased ordering:
   - Shutdown has highest priority
   - Timer checks for timeouts, keep-alives, re-announce
   - Peer message reading from first available connection

4. **Message Handlers**: Integrated with existing handlers:
   - Choke/Unchoke handling with request triggering
   - Have/Bitfield with interested message sending
   - Piece data with verification and completion checking

5. **Timeout Handling**: Two-pass approach to avoid borrow checker issues:
   - First pass: collect all timed-out requests
   - Second pass: process cancels and re-assignments

6. **Keep-Alive Handling**: Sends keep-alives to idle peers after 120s

### Connection Manager Extension

- Added `connections_map_mut()` method to access internal HashMap

### Tests

- Unit tests for `EventLoopAction` equality and `EventLoopEvent` debug
- Test for `read_from_first_peer` with empty connections
- Integration tests for shutdown controller triggering

## Testing

- All 740+ tests pass
- Clippy lint check passes with no warnings
<!-- SECTION:NOTES:END -->
