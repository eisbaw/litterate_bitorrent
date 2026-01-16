---
id: task-0025
title: Implement progress display
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:29'
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
- [ ] #1 Progress display shows pieces completed out of total (e.g., '42/100 pieces')
- [ ] #2 Progress display shows percentage complete (e.g., '42.0%')
- [ ] #3 Progress display shows download speed in human-readable format (e.g., '1.5 MB/s')
- [ ] #4 Progress display shows estimated time remaining
- [ ] #5 Progress updates at reasonable interval (every 1-2 seconds, not spamming)
- [ ] #6 Progress display works correctly for torrents with many pieces (10000+)
- [ ] #7 Final completion message shows total time and average speed
<!-- AC:END -->
