---
id: task-0022
title: Write Metainfo chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The metainfo (.torrent) file is what users interact with. This chapter must explain the structure and purpose of each field, with special attention to the info dictionary and info hash computation. The reader should understand why the info hash is the 'identity' of a torrent and how it is computed. Single-file vs multi-file format differences need clear explanation. The parsing implementation should show how to extract typed data from BencodeValue.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Explains structure of .torrent files with field-by-field breakdown
- [ ] #2 Info hash computation is explained (SHA1 of bencoded info dict) with rationale
- [ ] #3 Single-file vs multi-file format differences are clearly documented
- [ ] #4 Parsing implementation walkthrough shows extraction and validation of each field
- [ ] #5 Reader understands why info hash is immutable and how it identifies a torrent
<!-- AC:END -->
