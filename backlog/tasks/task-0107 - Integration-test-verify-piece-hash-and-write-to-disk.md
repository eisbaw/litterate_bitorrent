---
id: task-0107
title: 'Integration test: verify piece hash and write to disk'
status: To Do
assignee: []
created_date: '2026-01-16 21:35'
labels:
  - phase4
  - integration
dependencies:
  - task-0091
  - task-0072
  - task-0099
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create an integration test that exercises the complete Phase 4 pipeline: given raw piece data with a known hash, verify the hash and write to the correct file location. This test validates that all Phase 4 components work together correctly. Use a small test file with known content and hash, simulate receiving blocks out of order, verify hash, and write to disk. This is the exit criteria for Phase 4.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Test creates a small multi-file torrent structure (2-3 files, 2-3 pieces)
- [ ] #2 Test simulates receiving blocks for one piece out of order
- [ ] #3 Test verifies piece hash matches expected SHA1
- [ ] #4 Test writes verified piece to disk at correct location
- [ ] #5 Test reads back from disk and verifies content matches original
- [ ] #6 Test covers piece spanning two files
- [ ] #7 Test covers hash mismatch detection (corrupted data)
<!-- AC:END -->
