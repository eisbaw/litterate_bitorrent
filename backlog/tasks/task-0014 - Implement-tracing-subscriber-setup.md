---
id: task-0014
title: Implement tracing-subscriber setup
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 10:07'
labels:
  - logging
  - tracing
  - phase7
dependencies:
  - task-0005
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Configure the tracing-subscriber for structured logging output. This provides observability into the client's behavior, essential for debugging protocol issues and understanding download progress. INFO level by default keeps output clean; DEBUG via --verbose flag reveals protocol details for troubleshooting.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tracing-subscriber initializes at application startup
- [x] #2 Default log level is INFO (shows piece completions, peer events, progress)
- [x] #3 With --verbose flag, log level switches to DEBUG (shows block requests, message parsing)
- [x] #4 Log output is human-readable with timestamps
- [x] #5 Log output goes to stderr (stdout reserved for future machine-readable output)
- [x] #6 Invalid RUST_LOG environment variable does not crash the application
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add init_logging function to nw/06-cli.nw using tracing-subscriber with env-filter
2. Configure defaults: INFO level normally, DEBUG with --verbose flag
3. Use stderr for output, preserve stdout for machine-readable output
4. Handle invalid RUST_LOG gracefully (fallback to default)
5. Add literate prose explaining the logging setup
6. Add tests for logging initialization
7. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented init_logging function in nw/06-cli.nw with the following features:

- Uses tracing-subscriber with EnvFilter for flexible log level configuration
- Default log level is INFO, switches to DEBUG when verbose=true
- RUST_LOG environment variable can override the default filter
- Invalid RUST_LOG values gracefully fall back to the default (no crash)
- Log output goes to stderr to preserve stdout for machine-readable output
- Human-readable format with timestamps, targets, and log levels

The implementation includes literate prose explaining the logging strategy and design decisions.

Tests verify:
- Default filter is INFO when verbose=false
- Default filter is DEBUG when verbose=true
- Invalid RUST_LOG values fall back to default
- Valid RUST_LOG values are respected
- RUST_LOG takes precedence over verbose flag

All 443 tests pass including 5 new logging-related tests.
<!-- SECTION:NOTES:END -->
