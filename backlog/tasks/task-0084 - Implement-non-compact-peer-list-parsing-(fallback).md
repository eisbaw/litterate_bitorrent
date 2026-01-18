---
id: task-0084
title: Implement non-compact peer list parsing (fallback)
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:32'
updated_date: '2026-01-18 02:14'
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
- [x] #1 Function takes BencodeValue (list), returns Result<Vec<SocketAddr>, TrackerError>
- [x] #2 Each dict in list must have ip (string) and port (integer) keys
- [x] #3 peer id key is optional and ignored (we only need address)
- [x] #4 IP string parsed via std::net for IPv4 (and optionally IPv6)
- [x] #5 Missing ip or port in any dict returns error with context
- [x] #6 Unit test: parse list of 2 peer dicts into correct addresses
- [x] #7 Unit test: parse empty list returns empty vec
- [x] #8 Unit test: dict missing ip field returns descriptive error
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add prose section explaining non-compact (dictionary) peer format from BEP-3
2. Implement parse_dictionary_peers() function that:
   - Takes BencodeValue (list of dicts)
   - Extracts ip (string) and port (integer) from each dict
   - Parses IP string via std::net
   - Returns Result<Vec<SocketAddr>, TrackerError>
3. Add unit tests:
   - Parse list of 2 peer dicts
   - Parse empty list returns empty vec
   - Dict missing ip field returns descriptive error
4. Add code reference in module structure
5. Run just tangle, just lint, just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented non-compact peer list parsing:
- parse_dictionary_peers() takes BencodeValue list
- Parses ip (string) and port (integer) from each dict
- Supports IPv4 and IPv6 via std::net
- Ignores optional peer id
- Returns descriptive errors with peer index
- Added 17 comprehensive unit tests
<!-- SECTION:NOTES:END -->
