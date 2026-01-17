---
id: task-0025
title: Implement progress display
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 23:05'
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
Implemented progress display for BitTorrent downloads.

## Changes:
- Added `ProgressDisplay` variant to `EventLoopEvent` enum
- Added `PROGRESS_DISPLAY_INTERVAL_SECS` constant (2 seconds)
- Added progress tracking fields to `EventLoopContext`:
  - `last_progress_bytes` - bytes at last display
  - `last_progress_time` - time of last display
  - `download_start_time` - for total elapsed calculation
- Added `handle_progress_display()` handler function
- Added `format_speed()` for human-readable speed (B/s, KB/s, MB/s, GB/s)
- Added `format_duration()` for human-readable ETA (seconds, minutes, hours, days)
- Updated completion message to use consistent formatting

## Progress Output Format:
```
42/100 pieces (42.0%) - 1.5 MB/s - ETA: 2m 30s
```

## Completion Message Format:
```
Download complete\! 1.24 GB in 5m 30s (avg 3.8 MB/s) - 100/100 pieces verified
```

## Testing:
- Unit tests for `format_speed` at all unit boundaries
- Unit tests for `format_duration` including edge cases
- Verified progress interval is within reasonable bounds (1-5 seconds)
<!-- SECTION:NOTES:END -->
