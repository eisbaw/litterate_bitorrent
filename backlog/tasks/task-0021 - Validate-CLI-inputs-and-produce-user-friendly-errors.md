---
id: task-0021
title: Validate CLI inputs and produce user-friendly errors
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:29'
labels:
  - cli
  - validation
  - errors
  - phase7
dependencies:
  - task-0005
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Validate that CLI inputs are valid before starting the download. Early validation with clear error messages prevents confusing failures later in the process. This is the fail-fast principle applied to user input.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Torrent file path is checked for existence before parsing
- [ ] #2 Torrent file path produces error: 'Torrent file not found: /path/to/missing.torrent'
- [ ] #3 Output directory is created if it does not exist (with parent directories)
- [ ] #4 Permission denied on output directory produces clear error message
- [ ] #5 Invalid port number (not 1-65535) produces error message
- [ ] #6 Port below 1024 without root produces warning about privileged ports
- [ ] #7 All validation errors include the problematic value in the message
<!-- AC:END -->
