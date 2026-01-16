---
id: task-0023
title: Implement bitfield/have message handler
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:30'
labels:
  - phase-6
  - message-handler
  - peer-protocol
dependencies:
  - task-0006
  - task-0015
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handles incoming bitfield and have messages from peers. Updates the peer's known piece availability in the strategy module. This is foundational for piece selection - we cannot request pieces until we know what peers have.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 handle_bitfield() parses bitfield message and updates peer's piece availability
- [ ] #2 handle_have() updates single piece availability for a peer
- [ ] #3 Validates bitfield length matches expected piece count
- [ ] #4 Updates strategy module with new availability data
- [ ] #5 Sets am_interested=true if peer has pieces we need, sends interested message
- [ ] #6 Unit test: bitfield parsing with various piece counts including edge cases
- [ ] #7 Unit test: have message updates availability correctly
<!-- AC:END -->
