---
id: task-0089
title: Implement request timeout handling and re-request
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:32'
updated_date: '2026-01-17 20:20'
labels:
  - phase-5
  - strategy
  - timeout
dependencies:
  - task-0083
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When a request times out, it must be cancelled with the original peer and re-requested from another peer that has the piece. This involves: removing from the original peer's queue, sending cancel message, finding alternative peer, and queuing the request there.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 handle_timeout() takes timed-out BlockRequest and peer list
- [x] #2 Removes request from original peer's queue
- [x] #3 Returns CancelMessage to send to original peer
- [x] #4 Selects alternative peer that has the piece and is not choking us
- [x] #5 Returns new BlockRequest for alternative peer, or None if no alternative available
- [x] #6 Marks block as not-in-flight so it can be re-requested
- [x] #7 Unit test: selects alternative peer that has the piece
- [x] #8 Unit test: returns None when no alternative peer available
- [x] #9 Unit test: generates correct cancel message format
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add TimeoutHandleResult struct to hold the result of timeout handling
2. Add handle_request_timeout function that:
   - Removes request from original peer queue
   - Creates Cancel message for original peer
   - Finds alternative peer using find_alternative_peer
3. Add find_alternative_peer helper that filters peers by has_piece and \!peer_choking
4. Add prose documentation in a new subsection explaining the timeout handling flow
5. Add unit tests for all acceptance criteria
6. Update client.rs chunk to include the new code
7. Run just check and cargo test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented timeout handling and re-request in nw/07-client.nw.

TimeoutHandleResult struct:
- cancel: Message (Cancel to send to original peer)
- alternative_peer: Option<SocketAddr>
- request: BlockRequest (for re-queuing)

handle_request_timeout(request, addr, queue, peers):
1. Removes request from original peer queue
2. Creates Cancel message
3. Finds alternative peer via find_alternative_peer

find_alternative_peer: Selects peer that has_piece AND not peer_choking AND not original.

Note: Caller marks block as not-in-flight in piece manager.

Tests added (7): cancel message creation, queue removal, alternative selection, none when unavailable.

All 725 tests pass.
<!-- SECTION:NOTES:END -->
