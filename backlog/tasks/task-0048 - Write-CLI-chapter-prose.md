---
id: task-0048
title: Write CLI chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-18 00:55'
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
Added comprehensive prose to nw/06-cli.nw covering the user-facing interface:

## Added Sections

**Progress Display** (AC #1, #2)
- What the user sees: piece count, percentage, speed, ETA format
- Implementation strategy: CLI initializes logging, event loop handles display
- Speed calculation: instantaneous vs average approach with formula
- Unit formatting thresholds (B/s, KB/s, MB/s, GB/s)
- Completion message format

**Graceful Shutdown** (AC #3)
- Problems with abrupt termination (tracker, peers, disk)
- Two-press pattern: first Ctrl+C graceful, second forces exit
- What user sees during shutdown sequence
- Architecture: ShutdownController, signal handler, event loop, cleanup
- Exit code 130 standard

**Usage Examples** (AC #4)
- Basic download
- Output directory with -o/--output
- Custom port with -p/--port
- Verbose logging with -v/--verbose
- Fine-grained RUST_LOG control
- Combining options
- Help output
- Scripting and automation (exit codes, stderr vs stdout)

## Verification
- just tangle: SUCCESS
- just weave: SUCCESS (677 page PDF)
- just check: SUCCESS
- just test: SUCCESS (799 tests pass)
<!-- SECTION:NOTES:END -->
