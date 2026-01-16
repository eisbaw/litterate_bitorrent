---
id: task-0090
title: Unified peer extraction with format detection
status: To Do
assignee: []
created_date: '2026-01-16 21:32'
labels:
  - tracker
  - parsing
dependencies:
  - task-0077
  - task-0084
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Combine compact and non-compact peer parsing into a single function that auto-detects the format. If peers is bytes, use compact parser. If peers is a list, use non-compact parser. This provides a clean interface for the tracker module: pass in raw peers field, get back list of socket addresses regardless of format.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Function takes BencodeValue (peers field), returns Result<Vec<SocketAddr>, TrackerError>
- [ ] #2 Detects bytes type and delegates to compact parser
- [ ] #3 Detects list type and delegates to non-compact parser
- [ ] #4 Returns error with descriptive message for unexpected type
- [ ] #5 Unit test: compact bytes correctly detected and parsed
- [ ] #6 Unit test: dict list correctly detected and parsed
- [ ] #7 Unit test: integer type returns format error
<!-- AC:END -->
