---
id: task-0026
title: Write Tracker chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The tracker is the discovery mechanism - how peers find each other. This chapter explains the HTTP tracker protocol: URL construction, request parameters (info_hash, peer_id, port, uploaded, downloaded, left, event), and response parsing. The compact peer format (BEP-23) needs clear byte-level explanation. Announce intervals and the re-announce cycle should be covered. Reader should understand why tracker-based discovery works and its limitations (single point of failure, privacy).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Announce URL construction is explained with all parameters documented
- [ ] #2 HTTP request/response cycle is walked through with example
- [ ] #3 Compact peer format (BEP-23) has byte-level diagram and parsing explanation
- [ ] #4 Announce intervals and re-announce cycle are documented
- [ ] #5 Limitations of tracker-based discovery are acknowledged (motivates DHT for future)
<!-- AC:END -->
