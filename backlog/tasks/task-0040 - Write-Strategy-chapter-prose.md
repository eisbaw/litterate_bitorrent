---
id: task-0040
title: Write Strategy chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 00:37'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Strategy determines download efficiency. This chapter explains peer selection criteria, piece selection algorithms (especially rarest-first and why it improves swarm health), and request pipelining (10-16 concurrent requests per peer for throughput). Endgame mode can be mentioned as optional optimization. The reader should understand WHY these strategies matter - not just what they are, but how they affect download speed and swarm resilience.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Peer selection criteria documented (which peers to connect to, which to prioritize)
- [x] #2 Rarest-first piece selection explained with rationale (improves availability, prevents rare piece extinction)
- [x] #3 Request pipelining explained with queue depth justification (latency hiding, throughput)
- [x] #4 Endgame mode documented as optional optimization for final pieces
- [x] #5 Reader understands tradeoffs between strategies and their impact on swarm health
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a new \section{Download Strategy} before the Piece Availability section to introduce strategy concepts
2. Add \subsection{Peer Selection Criteria} explaining which peers to connect to and prioritize
3. Enhance existing \subsection{Rarest-First Piece Selection} with deeper rationale on swarm health
4. Add \subsection{Request Pipelining} with queue depth math and latency hiding explanation
5. Add \subsection{Endgame Mode} documenting the optimization for final pieces
6. Add \subsection{Strategy Tradeoffs} discussing impact on swarm health
7. Run just tangle and just weave to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added comprehensive Strategy chapter prose:
- Peer selection criteria with connection limits and prioritization
- Rarest-first explanation with extinction prevention math
- Request pipelining with BDP formula and worked examples
- Endgame mode documentation
- Strategy tradeoffs and swarm health section
<!-- SECTION:NOTES:END -->
