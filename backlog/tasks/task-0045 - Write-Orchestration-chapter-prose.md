---
id: task-0045
title: Write Orchestration chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Orchestration ties everything together into a working client. This chapter explains the main event loop structure, peer connection state machine, simplified choking algorithm, completion detection, and graceful shutdown (sending event=stopped to tracker). The reader should see how all previous components integrate. Code walkthrough should emphasize async/await patterns and tokio usage. Error handling and recovery strategies need coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Main event loop structure explained with async/await patterns
- [ ] #2 Peer connection state machine documented with state transitions
- [ ] #3 Choking algorithm explained (simplified: unchoke interested peers with good rates)
- [ ] #4 Completion detection logic documented
- [ ] #5 Graceful shutdown process documented (event=stopped to tracker)
- [ ] #6 Error handling and recovery strategies explained (retry, graceful degradation)
<!-- AC:END -->
