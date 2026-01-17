---
id: task-0021
title: Validate CLI inputs and produce user-friendly errors
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 10:13'
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
- [x] #1 Torrent file path is checked for existence before parsing
- [x] #2 Torrent file path produces error: 'Torrent file not found: /path/to/missing.torrent'
- [x] #3 Output directory is created if it does not exist (with parent directories)
- [x] #4 Permission denied on output directory produces clear error message
- [x] #5 Invalid port number (not 1-65535) produces error message
- [x] #6 Port below 1024 without root produces warning about privileged ports
- [x] #7 All validation errors include the problematic value in the message
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add ValidationError enum with thiserror for user-friendly messages
2. Add validate_args function that checks:
   - Torrent file exists and is readable
   - Output directory exists or can be created
   - Port is in valid range (1-65535), warn about privileged ports
3. Write literate prose explaining validation philosophy
4. Add comprehensive tests for each validation case
5. Verify with just check && just test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented semantic validation for CLI arguments with user-friendly error messages.

## Changes

- Added `ValidationError` enum with `thiserror` for clear error messages:
  - `TorrentNotFound`: When torrent file does not exist
  - `TorrentNotReadable`: When file exists but cannot be read
  - `OutputDirectoryError`: When directory creation/access fails
  - `InvalidPort`: When port is 0 (not bindable)

- Added `ValidationWarning` enum for non-fatal issues:
  - `PrivilegedPort`: Warning for ports below 1024 requiring root

- Added `validate_args()` function that performs:
  1. Torrent file existence and readability check
  2. Output directory creation (with parent dirs) or write permission check
  3. Port validity check (1-65535, warn on privileged)

## Testing
- Added 16 new tests covering all validation scenarios
- Edge cases tested: port boundaries (0, 1023, 1024, 65535)
- Error messages verified to include problematic values

## Files Modified
- `nw/06-cli.nw`: Added validation section with literate prose
- `Cargo.toml`: Added `tempfile` dev-dependency for tests
<!-- SECTION:NOTES:END -->
