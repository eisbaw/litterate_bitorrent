---
id: task-0037
title: Implement block request scheduler
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 17:11'
labels:
  - phase-6
  - request-scheduling
  - data-path
dependencies:
  - task-0027
  - task-0023
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Schedules block requests to unchoked peers. Implements request pipelining (10-16 concurrent requests per peer as per PRD) to maximize throughput. Requests pieces selected by the strategy module.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 request_blocks() sends requests to unchoked peers up to pipeline limit
- [x] #2 Respects per-peer pipeline limit (configurable, default 10)
- [x] #3 Queries strategy module for next piece to request
- [x] #4 Tracks pending requests with timestamps for timeout detection
- [x] #5 Implements 30-second request timeout per PRD
- [x] #6 Timed-out requests are cancelled and reassigned to other peers
- [x] #7 Does not request blocks already pending from another peer (except endgame)
- [x] #8 Unit test: pipeline limit is respected
- [x] #9 Unit test: timeout triggers request reassignment
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented RequestScheduler struct in nw/05-pieces.nw with the following components:

- SchedulerConfig: Configuration for pipeline limit (default 10) and request timeout (default 30s)
- BlockKey: Hash key for identifying blocks by (piece_index, offset)
- PendingRequest: Tracks in-flight requests with their BlockRequest and RequestState
- RequestScheduler: Central coordinator with methods:
  - register_request(): Add new block request, rejects duplicates
  - complete_request(): Mark block as received, decrement peer count
  - cancel_request(): Cancel individual request
  - cancel_peer_requests(): Cancel all requests to a peer (for disconnect/choke)
  - collect_timed_out(): Find and remove timed-out requests for reassignment
  - in_flight_offsets_for_piece(): Get offsets pending for a piece
  - request_blocks(): Combines rarest-first selection with block selection
  - available_slots(): Check remaining pipeline capacity for a peer
  - is_block_pending(): Duplicate avoidance check

Integrates with existing select_rarest() and select_next_blocks() functions.

Test coverage includes:
- Pipeline limit enforcement
- Duplicate request rejection
- Request completion/cancellation accounting
- Peer request cancellation
- Timeout detection (zero-duration timeout testing)
- Request reassignment workflow
- Rarest-first integration
- Block selection integration
<!-- SECTION:NOTES:END -->
