---
id: task-0122
title: Add JSON logging mode for monitoring
status: To Do
assignee: []
created_date: '2026-01-18 09:48'
labels:
  - feature
  - cli
  - observability
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add --json-log CLI flag for machine-readable structured logging. Include fields like pieces downloaded, average speed, active peer count, tracker interval. Useful for monitoring long-running downloads and integration with external tools.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Add --json-log flag to CLI args
- [ ] #2 When enabled, emit JSON objects to stderr instead of human-readable logs
- [ ] #3 Include fields: timestamp, event_type, pieces_done, pieces_total, download_speed_bps, upload_speed_bps, peer_count
- [ ] #4 Progress events emitted every 2 seconds (same as current progress display)
- [ ] #5 Unit test: JSON output is valid JSON and contains required fields
<!-- AC:END -->
