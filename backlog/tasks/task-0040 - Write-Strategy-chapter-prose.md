---
id: task-0040
title: Write Strategy chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:28'
updated_date: '2026-01-18 00:32'
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
Added comprehensive download strategy prose to nw/05-pieces.nw:

- New \section{Download Strategy} introducing the three pillars: peer selection, piece selection, and request scheduling
- \subsection{Peer Selection Criteria} covering connection limits, peer prioritization (upload speed, piece availability, rare piece holders, geographic proximity), and optimistic unchoking
- Enhanced \subsection{Rarest-First Piece Selection} with:
  - Piece extinction math showing P(extinction) = p^a
  - Game-theoretic benefits explaining the virtuous cycle
  - Comparison with alternative strategies (random, sequential, most-common-first)
- New \subsection{Request Pipelining} with:
  - Latency problem explanation with throughput formulas
  - Bandwidth-delay product (BDP) math for optimal queue depth
  - Example calculations for 10Mbps/100ms and 100Mbps/50ms scenarios
  - Practical considerations (memory, fairness, peer limits)
- New \subsection{Endgame Mode} documenting:
  - When to enter endgame mode
  - Mechanics (duplicate requests, cancel messages)
  - Tradeoffs and implementation status note
- New \subsection{Strategy Tradeoffs and Swarm Health} covering:
  - Individual vs collective optimization
  - Free-rider problem and tit-for-tat
  - Pipeline depth fairness
  - Seeder economics
  - Network effects
  - Summary of healthy swarm properties

All content uses LaTeX formatting with formulas, itemize/enumerate/description environments, and references code chunks with [[chunk name]] syntax. Both just tangle and just weave succeed, and all 799 cargo tests pass.
<!-- SECTION:NOTES:END -->
