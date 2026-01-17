---
id: task-0022
title: Write Metainfo chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-17 23:50'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The metainfo (.torrent) file is what users interact with. This chapter must explain the structure and purpose of each field, with special attention to the info dictionary and info hash computation. The reader should understand why the info hash is the 'identity' of a torrent and how it is computed. Single-file vs multi-file format differences need clear explanation. The parsing implementation should show how to extract typed data from BencodeValue.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Explains structure of .torrent files with field-by-field breakdown
- [x] #2 Info hash computation is explained (SHA1 of bencoded info dict) with rationale
- [x] #3 Single-file vs multi-file format differences are clearly documented
- [x] #4 Parsing implementation walkthrough shows extraction and validation of each field
- [x] #5 Reader understands why info hash is immutable and how it identifies a torrent
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze existing prose and code chunks in 02-metainfo.nw
2. Enhance opening section with deeper context about .torrent file purpose
3. Add detailed field-by-field breakdown with BEP 3 alignment
4. Expand info hash section with immutability rationale and cryptographic significance
5. Improve single-file vs multi-file documentation with concrete examples
6. Add parsing walkthrough prose connecting each code chunk
7. Verify just tangle and just weave succeed
8. Update acceptance criteria
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Enhanced nw/02-metainfo.nw with comprehensive prose covering:

- **Why Metainfo Matters**: Explains the four key problems solved (discovery, identity, integrity, structure)
- **Top-Level Structure**: Field-by-field breakdown with BEP 3 alignment
- **The Info Dictionary**: Detailed explanation of all info dict fields
- **Info Hash sections**: Why it matters, immutability rationale, and computation details
- **Single-File vs Multi-File**: Separate subsections with concrete bencode examples, plus explanation of how pieces span file boundaries
- **Implementation Overview**: 6-step parsing walkthrough from bytes to Metainfo struct

All tests pass (32/32 metainfo tests). Both just tangle and just weave succeed.
PDF generated: 638 pages.
<!-- SECTION:NOTES:END -->
