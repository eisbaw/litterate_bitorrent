---
id: task-0090
title: Unified peer extraction with format detection
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:32'
updated_date: '2026-01-18 02:19'
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
- [x] #1 Function takes BencodeValue (peers field), returns Result<Vec<SocketAddr>, TrackerError>
- [x] #2 Detects bytes type and delegates to compact parser
- [x] #3 Detects list type and delegates to non-compact parser
- [x] #4 Returns error with descriptive message for unexpected type
- [x] #5 Unit test: compact bytes correctly detected and parsed
- [x] #6 Unit test: dict list correctly detected and parsed
- [x] #7 Unit test: integer type returns format error
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add <<unified peer extraction>> code chunk with extract_peers() function that matches on BencodeValue variants
2. Add prose explaining the format detection strategy
3. Add <<unified peer extraction tests>> test chunk with 3 tests (compact, dict, error)
4. Update main file assembly to include new chunks
5. Run just tangle, just lint, just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added extract_peers() function as unified interface for peer extraction with automatic format detection.

Changes to nw/03-tracker.nw:
- Added <<unified peer extraction>> code chunk with extract_peers() function
- Added <<unified peer extraction tests>> test chunk with 8 comprehensive tests
- Added prose explaining the format detection strategy
- Updated file assembly to include new chunks

Implementation details:
- Function matches on BencodeValue::Bytes (compact BEP-23) or BencodeValue::List (dictionary BEP-3)
- Delegates to existing parse_compact_peers() and parse_dictionary_peers() functions
- Returns TrackerError::TrackerInvalidResponse for unexpected types (Integer, Dict at top level)
- Errors from underlying parsers are propagated transparently

Tests verify:
- Compact format detection and parsing
- Dictionary format detection and parsing
- Integer type error with descriptive message
- Top-level dict error
- Empty bytes/list edge cases
- Error propagation from both parsers
<!-- SECTION:NOTES:END -->
