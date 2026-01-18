---
id: task-0048
title: Write CLI chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-18 00:58'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The CLI is the user-facing interface. This chapter explains argument parsing with clap, progress display implementation, and Ctrl+C handling for graceful shutdown. While shorter than other chapters, it should show how the user interacts with all the underlying machinery. The reader should see a complete picture of how to use the client.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CLI argument parsing documented with clap usage
- [x] #2 Progress display implementation explained
- [x] #3 Ctrl+C handling for graceful shutdown documented
- [x] #4 Usage examples provided showing typical invocation
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add section on Progress Display explaining how it works (referencing 07-client.nw implementation)
2. Add section on Graceful Shutdown explaining Ctrl+C handling (referencing 07-client.nw)
3. Add Usage Examples section with typical invocations
4. Verify with just tangle && just weave
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added comprehensive CLI chapter prose:
- Progress display with format explanation and speed calculation
- Graceful shutdown with two-press Ctrl+C pattern
- Usage examples for common invocations
- Scripting guidance with exit codes
<!-- SECTION:NOTES:END -->
