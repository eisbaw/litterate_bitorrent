---
id: task-0034
title: Write Piece Management chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Piece management bridges the protocol to disk. This chapter explains piece and block geometry: piece_length from metainfo, BLOCK_SIZE constant (16384 bytes), blocks_per_piece calculation, and handling the last piece. Byte stream to file mapping for multi-file torrents needs careful explanation with diagrams. SHA1 verification after receiving all blocks is critical for integrity. Disk I/O strategy (seek+write, sparse files) should be explained with rationale for pedagogical choices.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Piece and block geometry explained with calculations (piece_length, BLOCK_SIZE, blocks_per_piece)
- [ ] #2 Last piece handling documented (may be smaller than piece_length)
- [ ] #3 Byte stream to file mapping for multi-file torrents has diagram and algorithm
- [ ] #4 SHA1 verification process explained with emphasis on integrity guarantees
- [ ] #5 Disk I/O strategy documented: seek+write approach, sparse files, no pre-allocation
<!-- AC:END -->
