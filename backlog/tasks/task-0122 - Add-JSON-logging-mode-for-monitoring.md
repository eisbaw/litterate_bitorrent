---
id: task-0122
title: Add JSON logging mode for monitoring
status: Done
assignee:
  - '@claude'
created_date: '2026-01-18 09:48'
updated_date: '2026-01-18 11:27'
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
- [x] #1 Add --json-log flag to CLI args
- [x] #2 When enabled, emit JSON objects to stderr instead of human-readable logs
- [x] #3 Include fields: timestamp, event_type, pieces_done, pieces_total, download_speed_bps, upload_speed_bps, peer_count
- [x] #4 Progress events emitted every 2 seconds (same as current progress display)
- [x] #5 Unit test: JSON output is valid JSON and contains required fields
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Implementation Summary

Added JSON logging mode for machine-readable progress output, enabling integration with monitoring tools.

### Changes Made

1. **Cargo.toml**: Added serde, serde_json, and chrono dependencies for JSON serialization and timestamps

2. **nw/06-cli.nw**: Added `--json-log` boolean flag to Args struct with documentation

3. **nw/07-client.nw**:
   - Added `json_log` field to EventLoopContext struct
   - Created `ProgressEvent` struct with Serialize derive for JSON output
   - Modified `handle_progress_display()` to emit JSON to stderr when json_log is enabled
   - Added unit tests for JSON serialization (5 tests covering validity, field values, timestamp format, zero values, large values)

4. **nw/00-main.nw**: Passed json_log flag through EventLoopContext initialization

### JSON Output Format

```json
{
  "timestamp": "2024-01-18T12:34:56+00:00",
  "event_type": "progress",
  "pieces_done": 42,
  "pieces_total": 100,
  "percentage": 42.0,
  "download_speed_bps": 1048576,
  "upload_speed_bps": 0,
  "peer_count": 5
}
```

### Testing

- All 951 unit tests pass
- Clippy lint checks pass
- CLI help shows new --json-log flag correctly
<!-- SECTION:NOTES:END -->
