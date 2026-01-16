---
id: task-0102
title: Define Metainfo and FileInfo structs
status: To Do
assignee: []
created_date: '2026-01-16 21:34'
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
- [ ] #1 Metainfo struct has fields: announce (String), info_hash ([u8; 20]), piece_length (u32), piece_hashes (Vec<[u8; 20]>), total_length (u64), files (Vec<FileInfo>), name (String)
- [ ] #2 FileInfo struct has fields: length (u64), path (PathBuf)
- [ ] #3 MetainfoError type defined with MissingField, InvalidPieceLength, InvalidPieceHashes variants
- [ ] #4 Code compiles with cargo check
<!-- AC:END -->
