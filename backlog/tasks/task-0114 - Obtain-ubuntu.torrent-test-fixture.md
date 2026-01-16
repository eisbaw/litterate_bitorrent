---
id: task-0114
title: Obtain ubuntu.torrent test fixture
status: To Do
assignee: []
created_date: '2026-01-16 21:38'
labels:
  - testing
  - fixture
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Download a stable Ubuntu ISO torrent file to use as integration test fixture. Choose an LTS version for stability. Store in tests/fixtures/ and document the expected hash values for verification.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 tests/fixtures/ directory exists
- [ ] #2 ubuntu.torrent file downloaded from official Ubuntu source
- [ ] #3 Document expected info_hash (hex) in a comment or test
- [ ] #4 Document expected piece_count
- [ ] #5 Document expected total_length
- [ ] #6 File committed to repository
<!-- AC:END -->
