---
id: task-0005
title: Define CLI arguments with clap
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels:
  - cli
  - clap
  - phase7
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define the command-line interface for the literate-bittorrent client using clap. This establishes the user-facing contract: how users invoke the client, what options are available, and provides automatic help text generation. A well-designed CLI is the first point of contact with users.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 CLI accepts required positional argument for torrent file path
- [ ] #2 CLI accepts -o/--output option for output directory (defaults to current directory)
- [ ] #3 CLI accepts -p/--port option for listen port (defaults to 6881)
- [ ] #4 CLI accepts -v/--verbose flag for DEBUG logging
- [ ] #5 CLI generates --help output with usage examples
- [ ] #6 Invalid arguments produce helpful error messages (e.g., 'missing required argument: TORRENT')
<!-- AC:END -->
