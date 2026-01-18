---
id: task-0034
title: Write Piece Management chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 00:23'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Piece management bridges the protocol to disk. This chapter explains piece and block geometry: piece_length from metainfo, BLOCK_SIZE constant (16384 bytes), blocks_per_piece calculation, and handling the last piece. Byte stream to file mapping for multi-file torrents needs careful explanation with diagrams. SHA1 verification after receiving all blocks is critical for integrity. Disk I/O strategy (seek+write, sparse files) should be explained with rationale for pedagogical choices.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Piece and block geometry explained with calculations (piece_length, BLOCK_SIZE, blocks_per_piece)
- [x] #2 Last piece handling documented (may be smaller than piece_length)
- [x] #3 Byte stream to file mapping for multi-file torrents has diagram and algorithm
- [x] #4 SHA1 verification process explained with emphasis on integrity guarantees
- [x] #5 Disk I/O strategy documented: seek+write approach, sparse files, no pre-allocation
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added comprehensive Piece Management chapter prose:
- Piece/block geometry with formulas and worked examples
- Last piece handling with edge case diagrams
- Byte stream to file mapping diagram
- SHA1 verification with security model
- Disk I/O strategy documentation
<!-- SECTION:NOTES:END -->
