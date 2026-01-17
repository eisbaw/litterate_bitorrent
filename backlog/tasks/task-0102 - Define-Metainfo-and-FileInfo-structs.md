---
id: task-0102
title: Define Metainfo and FileInfo structs
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:34'
updated_date: '2026-01-17 00:07'
labels:
  - metainfo
  - foundation
dependencies:
  - task-0071
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Data structures representing parsed .torrent file content. Metainfo holds announce URL, info hash, piece hashes, and file information. FileInfo represents individual files in multi-file torrents. Design follows PRD specification.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Metainfo struct has fields: announce (String), info_hash ([u8; 20]), piece_length (u32), piece_hashes (Vec<[u8; 20]>), total_length (u64), files (Vec<FileInfo>), name (String)
- [x] #2 FileInfo struct has fields: length (u64), path (PathBuf)
- [x] #3 MetainfoError type defined with MissingField, InvalidPieceLength, InvalidPieceHashes variants
- [x] #4 Code compiles with cargo check
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create nw/02-metainfo.nw with LaTeX prose explaining torrent metainfo structure
2. Define FileInfo struct with length (u64) and path (PathBuf)
3. Define Metainfo struct with all required fields per PRD
4. Define MetainfoError with MissingField, InvalidPieceLength, InvalidPieceHashes variants using thiserror
5. Update nw/00-main.nw lib.rs chunk to add pub mod metainfo
6. Update justfile tangle recipe to extract metainfo.rs
7. Verify compilation with just tangle && just check
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented Metainfo and FileInfo structs in nw/02-metainfo.nw following the literate programming style.

Key decisions:
- FileInfo uses PathBuf for path field (handles non-UTF8 filenames in torrent protocol)
- Metainfo uses [u8; 20] for info_hash (fixed size prevents misuse)
- piece_length is u32 (matches protocol 32-bit piece index)
- total_length is u64 (supports torrents > 4GB)

Files changed:
- nw/02-metainfo.nw: New literate source with prose explaining torrent metainfo structure
- nw/00-main.nw: Added pub mod metainfo to lib.rs chunk
- justfile: Added notangle command for metainfo.rs

Verified: just tangle && just check && just test && just lint all pass.
<!-- SECTION:NOTES:END -->
