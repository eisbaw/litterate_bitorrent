---
id: task-0040
title: Write Strategy chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Strategy determines download efficiency. This chapter explains peer selection criteria, piece selection algorithms (especially rarest-first and why it improves swarm health), and request pipelining (10-16 concurrent requests per peer for throughput). Endgame mode can be mentioned as optional optimization. The reader should understand WHY these strategies matter - not just what they are, but how they affect download speed and swarm resilience.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Peer selection criteria documented (which peers to connect to, which to prioritize)
- [ ] #2 Rarest-first piece selection explained with rationale (improves availability, prevents rare piece extinction)
- [ ] #3 Request pipelining explained with queue depth justification (latency hiding, throughput)
- [ ] #4 Endgame mode documented as optional optimization for final pieces
- [ ] #5 Reader understands tradeoffs between strategies and their impact on swarm health
<!-- AC:END -->
