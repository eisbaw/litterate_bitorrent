---
id: task-0034
title: Write Piece Management chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 00:20'
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
Added comprehensive prose to nw/05-pieces.nw explaining piece management concepts:

- **Piece and Block Geometry Overview**: New section with formulas for num_pieces, blocks_per_piece, piece_size, and block_size calculations. Includes worked example showing 1,000,000 byte torrent with 4 pieces.

- **Last Piece and Last Block Handling**: Detailed explanation with verbatim diagram showing how the last piece is often smaller, with formulas for calculating actual sizes.

- **Byte Stream to File Mapping**: New section with ASCII diagram showing 3-file torrent mapped to virtual byte stream. Explains how piece 1 spans file boundaries with FileSlice mapping algorithm.

- **SHA1 Verification**: Expanded security model section covering data integrity, decentralized trust, partial verification, and resume capability. Documents what happens when verification fails.

- **Disk I/O Strategy**: New section documenting sparse file pre-allocation, seek-and-write approach, and explicit non-use of write coalescing/caching for pedagogical clarity.

All prose uses LaTeX math mode for formulas and verbatim environments for diagrams. Tests pass: just tangle, just weave, and cargo test all succeed.
<!-- SECTION:NOTES:END -->
