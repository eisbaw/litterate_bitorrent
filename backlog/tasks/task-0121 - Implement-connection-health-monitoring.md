---
id: task-0121
title: Implement connection health monitoring
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:47'
updated_date: '2026-01-18 11:02'
labels:
  - reliability
  - performance
  - client
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add detection and removal of connections that haven't transmitted useful data for 180+ seconds. Currently dead connections stay open and waste resources. Track peer quality metrics to prioritize healthy connections.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Track last useful activity timestamp per peer (block received, bitfield update)
- [x] #2 Add periodic health check in event loop (every 30 seconds)
- [x] #3 Disconnect peers with no useful activity for 180+ seconds
- [x] #4 Log peer disconnection reason at INFO level
- [x] #5 Replace disconnected unhealthy peers with fresh ones from pool
- [x] #6 Unit test: stale peer detection triggers disconnect
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented connection health monitoring for stale peer detection.

**Changes:**
- Added `last_useful_activity` field to `PeerConnection` struct to track when a peer last provided useful data
- Added `mark_useful_activity()` method to update this timestamp
- Added `PEER_STALE_TIMEOUT_SECS` (180s) and `STALE_PEER_CHECK_INTERVAL_SECS` (30s) constants
- Added `StalePeerCheck` event to `EventLoopEvent` enum
- Updated message handlers to call `mark_useful_activity()` on:
  - `Message::Unchoke` - allows requesting data
  - `Message::Have` - piece availability update
  - `Message::Bitfield` - initial peer state
  - `Message::Piece` - actual block data
- Added `handle_stale_peer_check()` function that runs every 30 seconds
- Added unit tests for the new functionality

**Key insight:** The distinction between "idle" and "stale" is important:
- Idle timeout (existing): no messages at all, including keep-alives (dead connection)
- Stale timeout (new): no useful data, but may be sending keep-alives (alive but useless)

Peers that only send keep-alives consume connection slots without contributing to download progress. This change detects and disconnects them to free slots for more productive peers.

**Files modified:** `nw/07-client.nw`
<!-- SECTION:NOTES:END -->
