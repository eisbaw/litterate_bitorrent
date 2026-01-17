---
id: task-0005
title: Define CLI arguments with clap
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:27'
updated_date: '2026-01-17 10:04'
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
- [x] #1 CLI accepts required positional argument for torrent file path
- [x] #2 CLI accepts -o/--output option for output directory (defaults to current directory)
- [x] #3 CLI accepts -p/--port option for listen port (defaults to 6881)
- [x] #4 CLI accepts -v/--verbose flag for DEBUG logging
- [x] #5 CLI generates --help output with usage examples
- [x] #6 Invalid arguments produce helpful error messages (e.g., 'missing required argument: TORRENT')
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/06-cli.nw with literate documentation and clap-based CLI
2. Define Args struct with clap derive for: torrent path, output dir, port, verbose flag
3. Add tests for argument parsing (valid args, defaults, errors)
4. Update nw/00-main.nw to include pub mod cli in lib.rs
5. Update justfile to tangle cli.rs from 06-cli.nw
6. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented CLI module using clap derive macros in nw/06-cli.nw.

## Changes
- Created nw/06-cli.nw with literate documentation explaining CLI design
- Args struct with clap derive for torrent path (required positional), output dir (-o, default "."), port (-p, default 6881), verbose flag (-v)
- Added pub mod cli to lib.rs in nw/00-main.nw
- Updated justfile to tangle cli.rs from 06-cli.nw

## Testing
- 24 unit tests covering all acceptance criteria
- Tests for valid args, defaults, error messages, help output
- All 438 tests pass including new CLI tests

## Files modified
- nw/06-cli.nw (new)
- nw/00-main.nw (added pub mod cli)
- justfile (added cli.rs tangle command)
<!-- SECTION:NOTES:END -->
