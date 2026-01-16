---
id: task-0014
title: Implement tracing-subscriber setup
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
updated_date: '2026-01-16 21:29'
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
- [ ] #1 tracing-subscriber initializes at application startup
- [ ] #2 Default log level is INFO (shows piece completions, peer events, progress)
- [ ] #3 With --verbose flag, log level switches to DEBUG (shows block requests, message parsing)
- [ ] #4 Log output is human-readable with timestamps
- [ ] #5 Log output goes to stderr (stdout reserved for future machine-readable output)
- [ ] #6 Invalid RUST_LOG environment variable does not crash the application
<!-- AC:END -->
