---
id: task-0097
title: Create announce function integrating all tracker components
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 08:54'
labels:
  - tracker
  - integration
dependencies:
  - task-0059
  - task-0090
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
High-level async function that performs a complete tracker announce: constructs URL, makes HTTP request, parses response, extracts peers. This is the main entry point for the tracker module. Returns a clean AnnounceResult with peers and metadata. Handles all error cases by mapping to appropriate TrackerError variants.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Async function takes Metainfo, TrackerRequest, returns Result<AnnounceResult, TrackerError>
- [x] #2 AnnounceResult struct contains: peers (Vec<SocketAddr>), interval (u64), seeders/leechers (Option<u64>)
- [x] #3 Internally calls URL construction, HTTP request, response parsing, peer extraction
- [x] #4 Logs announce URL at debug level, result summary at info level
- [x] #5 Errors from any stage propagate with appropriate type
- [x] #6 Function documented in nw/03-tracker.nw with narrative flow
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Define AnnounceResult struct with peers (Vec<SocketAddr>), interval (u64), seeders/leechers (Option<u64>)
2. Create async announce function taking Metainfo and TrackerRequest
3. Integrate: build_announce_url -> announce_http -> parse_tracker_response -> parse_compact_peers
4. Add debug logging for URL and info logging for result
5. Write literate prose explaining the announce flow
6. Add tests for function signature, error propagation, and result structure
7. Verify with nix-shell --run "just check && just test"
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented high-level announce() function integrating all tracker components:

- Added AnnounceResult struct with peers (Vec<SocketAddr>), interval (u64), and optional seeders/leechers
- Created async announce() function that orchestrates: build_announce_url -> announce_http -> parse_tracker_response -> parse_compact_peers
- Added debug logging for URL and info logging for result summary
- Wrote comprehensive literate prose explaining the announce flow in nw/03-tracker.nw
- Added tests for AnnounceResult construction and announce function signature

Files modified:
- nw/03-tracker.nw: Added announce result struct, announce function, and tests
- src/tracker.rs: Generated from tangling

All 226 tests pass.
<!-- SECTION:NOTES:END -->
