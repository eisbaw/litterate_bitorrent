---
id: task-0057
title: Add tracker error types to error module
status: To Do
assignee: []
created_date: '2026-01-16 21:29'
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
- [ ] #1 TrackerHttpError variant wraps reqwest::Error for HTTP-level failures
- [ ] #2 TrackerErrorResponse variant contains String from tracker failure reason
- [ ] #3 TrackerInvalidResponse variant for malformed bencode or missing fields
- [ ] #4 TrackerTimeout variant for 30s timeout exceeded
- [ ] #5 All variants implement std::error::Error via thiserror
- [ ] #6 Error messages include context (e.g., which field was missing)
- [ ] #7 Errors added to nw/03-tracker.nw or existing error module
<!-- AC:END -->
