---
id: task-0043
title: Implement simplified choking algorithm
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:31'
labels:
  - phase-6
  - choking
  - peer-protocol
dependencies:
  - task-0015
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implements our choking decisions toward peers. Per PRD, use simplified algorithm: unchoke all interested peers. We are a leecher, so we unchoke everyone to encourage reciprocity. This is simpler than full tit-for-tat but sufficient for downloading.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 unchoke_interested_peers() sends unchoke to all interested peers
- [ ] #2 Runs periodically (every 10 seconds) to catch new interested peers
- [ ] #3 Sends choke when peer becomes uninterested (optional optimization)
- [ ] #4 Logs choke/unchoke decisions at DEBUG level
- [ ] #5 Unit test: interested peers receive unchoke message
<!-- AC:END -->
