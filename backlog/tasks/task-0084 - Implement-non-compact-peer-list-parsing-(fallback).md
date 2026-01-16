---
id: task-0084
title: Implement non-compact peer list parsing (fallback)
status: To Do
assignee: []
created_date: '2026-01-16 21:32'
labels:
  - tracker
  - parsing
  - fallback
dependencies:
  - task-0069
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Some trackers do not support compact format and return peers as a list of dictionaries, each containing peer id, ip (as string), and port. This is the original format from BEP-3. While less efficient, we must support it as a fallback. The parser detects format by checking if peers value is bytes (compact) or list (non-compact).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Function takes BencodeValue (list), returns Result<Vec<SocketAddr>, TrackerError>
- [ ] #2 Each dict in list must have ip (string) and port (integer) keys
- [ ] #3 peer id key is optional and ignored (we only need address)
- [ ] #4 IP string parsed via std::net for IPv4 (and optionally IPv6)
- [ ] #5 Missing ip or port in any dict returns error with context
- [ ] #6 Unit test: parse list of 2 peer dicts into correct addresses
- [ ] #7 Unit test: parse empty list returns empty vec
- [ ] #8 Unit test: dict missing ip field returns descriptive error
<!-- AC:END -->
