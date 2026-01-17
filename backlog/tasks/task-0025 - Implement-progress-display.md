---
id: task-0025
title: Implement progress display
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 23:07'
labels:
  - cli
  - progress
  - ux
  - phase7
dependencies:
  - task-0014
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Display download progress to the user in a clear, informative format. Users need to know the download is progressing, how fast, and when it will complete. This provides the primary feedback loop during potentially long downloads.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Progress display shows pieces completed out of total (e.g., '42/100 pieces')
- [x] #2 Progress display shows percentage complete (e.g., '42.0%')
- [x] #3 Progress display shows download speed in human-readable format (e.g., '1.5 MB/s')
- [x] #4 Progress display shows estimated time remaining
- [x] #5 Progress updates at reasonable interval (every 1-2 seconds, not spamming)
- [x] #6 Progress display works correctly for torrents with many pieces (10000+)
- [x] #7 Final completion message shows total time and average speed
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add ProgressDisplay variant to EventLoopEvent enum
2. Add progress_interval timer in run_event_loop (2 seconds)
3. Add progress tracking fields to EventLoopContext (last_bytes, last_progress_time, start_time)
4. Create handle_progress_display() function that:
   - Gets current verified/total pieces
   - Calculates percentage
   - Calculates download speed from delta bytes/delta time
   - Estimates time remaining
   - Formats and outputs progress line
5. Add format_duration() helper for ETA formatting
6. Update final completion message in check_completion()
7. Write tests for progress calculations and formatting
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Progress display via tracing INFO. format_speed/format_duration helpers. 2-second interval.
<!-- SECTION:NOTES:END -->
