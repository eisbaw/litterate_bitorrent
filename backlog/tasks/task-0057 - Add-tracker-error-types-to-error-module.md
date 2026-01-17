---
id: task-0057
title: Add tracker error types to error module
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-17 08:10'
labels:
  - tracker
  - error-handling
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The tracker module needs specific error types to handle various failure modes: HTTP errors, tracker returning failure reason, invalid/unparseable responses, and timeouts. These errors should integrate with the project error taxonomy defined in the PRD. Good error messages help debugging tracker issues which are common during BitTorrent development.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 TrackerHttpError variant wraps reqwest::Error for HTTP-level failures
- [x] #2 TrackerErrorResponse variant contains String from tracker failure reason
- [x] #3 TrackerInvalidResponse variant for malformed bencode or missing fields
- [x] #4 TrackerTimeout variant for 30s timeout exceeded
- [x] #5 All variants implement std::error::Error via thiserror
- [x] #6 Error messages include context (e.g., which field was missing)
- [x] #7 Errors added to nw/03-tracker.nw or existing error module
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/03-tracker.nw with LaTeX prose explaining tracker errors
2. Define TrackerError enum with 4 variants using thiserror
3. Update nw/00-main.nw to add `pub mod tracker;` to lib.rs chunk
4. Update justfile to add notangle command for tracker.rs
5. Run `just tangle && just check` to verify compilation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Summary

Implemented tracker error types for the literate BitTorrent client as a new noweb chapter (nw/03-tracker.nw).

## Changes

- Created nw/03-tracker.nw with TrackerError enum containing 4 variants:
  - TrackerHttpError: wraps reqwest::Error with #[from] for automatic conversion
  - TrackerErrorResponse: contains tracker failure reason string
  - TrackerInvalidResponse: for malformed bencode or missing fields
  - TrackerTimeout: for 30s timeout as per PRD specification
- Updated nw/00-main.nw to add `pub mod tracker;` to lib.rs chunk
- Updated justfile to add notangle command for tracker.rs extraction
- All variants implement std::error::Error via thiserror derive
- Error messages include relevant context (field names, timeout duration)

## Testing

- Added 5 tests verifying error display messages and trait implementations
- All 159 tests pass
- Clippy passes with no warnings
<!-- SECTION:NOTES:END -->
