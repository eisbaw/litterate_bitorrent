---
id: task-0098
title: Write Introduction chapter prose
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 23:41'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Introduction is the reader's first contact with the project. It must answer 'What is BitTorrent?' at a conceptual level, provide a visual protocol overview diagram showing the relationships between tracker, peers, and pieces, and give a document roadmap so readers know what to expect. This sets the foundation for everything that follows - a reader should finish this chapter understanding why BitTorrent is interesting and how the rest of the document is organized.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Chapter explains what BitTorrent is at a conceptual level (distributed file sharing, swarms, seeders/leechers)
- [x] #2 Contains protocol overview diagram showing tracker, peers, and piece exchange relationships
- [x] #3 Includes document roadmap with brief descriptions of each subsequent chapter
- [x] #4 Reader unfamiliar with BitTorrent can understand the high-level architecture after reading
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add TikZ package to document preamble for diagram support
2. Write expanded "What is BitTorrent?" subsection explaining distributed file sharing, swarms, seeders/leechers
3. Create "How BitTorrent Works" subsection with protocol overview diagram using TikZ
4. Rewrite "Document Roadmap" subsection with brief descriptions of each chapter
5. Verify with just tangle and just weave
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Expanded Introduction section in nw/00-main.nw with:

- "What is BitTorrent?" subsection explaining:
  - The problem with traditional client-server downloads
  - The BitTorrent solution (peer-to-peer distribution)
  - Four key concepts: Pieces, Swarms, Seeders/Leechers, Trackers
  - Why BitTorrent succeeds (no single point of failure, scalability, incentives, verification)

- "How BitTorrent Works" subsection with:
  - TikZ diagram showing tracker, swarm of peers, torrent file, and data flow
  - Step-by-step explanation of the download lifecycle (parse torrent, announce, receive peers, exchange pieces)

- Rewritten "Document Roadmap" subsection with brief descriptions of all 8 chapters plus appendix

Technical changes:
- Added TikZ package to document preamble with positioning, arrows, shapes, fit, and backgrounds libraries
- Verified with just tangle (Rust extraction works)
- Verified with just weave (PDF generates successfully, 634 pages)
<!-- SECTION:NOTES:END -->
