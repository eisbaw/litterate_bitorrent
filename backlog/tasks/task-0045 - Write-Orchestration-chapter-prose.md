---
id: task-0045
title: Write Orchestration chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-18 00:45'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Orchestration ties everything together into a working client. This chapter explains the main event loop structure, peer connection state machine, simplified choking algorithm, completion detection, and graceful shutdown (sending event=stopped to tracker). The reader should see how all previous components integrate. Code walkthrough should emphasize async/await patterns and tokio usage. Error handling and recovery strategies need coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Main event loop structure explained with async/await patterns
- [x] #2 Peer connection state machine documented with state transitions
- [x] #3 Choking algorithm explained (simplified: unchoke interested peers with good rates)
- [x] #4 Completion detection logic documented
- [x] #5 Graceful shutdown process documented (event=stopped to tracker)
- [x] #6 Error handling and recovery strategies explained (retry, graceful degradation)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
$1. Review existing prose in 07-client.nw\n2. Add comprehensive async/await patterns explanation after Event Loop Design subsection\n3. Add peer connection state machine diagram (ASCII art) after PeerState struct\n4. Enhance choking algorithm explanation with tit-for-tat context\n5. Add completion detection flow diagram\n6. Add graceful shutdown sequence diagram\n7. Add error handling and recovery strategies section\n8. Run just tangle and just weave to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added comprehensive prose to nw/07-client.nw covering the orchestration layer:

- Added async/await patterns section explaining tokio::select\!, biased keyword, and timer management strategies
- Added peer connection state machine diagram showing lifecycle from DISCONNECTED through CONNECTING, HANDSHAKING, CONNECTED, to EXCHANGING
- Added 4-bit choke/interest state tables showing message triggers and state transitions
- Enhanced choking algorithm section with tit-for-tat explanation and comparison to simplified leecher-only strategy
- Added completion detection flow diagram showing piece verification cascade
- Added graceful shutdown sequence diagrams showing normal and force-exit paths
- Added error handling section covering error classification (transient, peer-specific, fatal), graceful degradation strategies, retry logic with exponential backoff, and logging philosophy

All code chunks remain unchanged. just tangle and just weave succeed. Pre-existing test failure in cli.rs is unrelated to these changes.
<!-- SECTION:NOTES:END -->
