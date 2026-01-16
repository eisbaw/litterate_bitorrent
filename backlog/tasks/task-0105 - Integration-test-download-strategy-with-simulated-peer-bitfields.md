---
id: task-0105
title: 'Integration test: download strategy with simulated peer bitfields'
status: To Do
assignee: []
created_date: '2026-01-16 21:35'
labels:
  - phase-5
  - strategy
  - integration-test
dependencies:
  - task-0076
  - task-0089
  - task-0095
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Verify the complete download strategy works end-to-end with simulated peers. This test validates rarest-first selection, request pipelining, and timeout handling work together correctly. Uses deterministic peer bitfields to verify expected behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Test fixture creates simulated swarm with 5+ peers having varying bitfields
- [ ] #2 Test verifies rarest piece is selected first
- [ ] #3 Test verifies request queue fills to capacity
- [ ] #4 Test verifies block completion removes request from queue
- [ ] #5 Test verifies piece completion triggers cancel messages to other peers
- [ ] #6 Test verifies timeout triggers re-request from alternative peer
- [ ] #7 Test passes with deterministic RNG for reproducibility
<!-- AC:END -->
