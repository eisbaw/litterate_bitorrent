---
id: task-0051
title: Write Appendices
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-18 01:03'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The appendices provide reference material that supports but does not interrupt the narrative. BEP references give readers primary sources for deeper understanding. Test vectors provide concrete examples for verification. Future work acknowledges limitations and points to extensions (DHT, uTP, PEX). The appendices should be useful for both readers wanting to go deeper and implementers wanting to test their understanding.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 BEP references appendix lists relevant BEPs with brief descriptions
- [x] #2 Test vectors appendix provides example bencoded values with expected outputs
- [x] #3 Test vectors include expected info hashes for known torrents
- [x] #4 Future work appendix discusses DHT, uTP, PEX, and other extensions not implemented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Research relevant BEPs for BitTorrent protocol
2. Write BEP references appendix with descriptions of each BEP
3. Create test vectors appendix with bencode examples
4. Add info hash examples for known public torrents
5. Write future work appendix covering DHT, uTP, PEX, magnet links
6. Verify document builds with just tangle and just weave
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Expanded nw/09-appendix.nw from a minimal 17-line file to a comprehensive 615-line appendix containing:

## BEP Reference (Section A)
- Core Protocol BEPs: BEP 3, 20, 23, 10 with descriptions
- Tracker Protocol BEPs: BEP 7, 15, 48
- Peer Protocol Extensions: BEP 5, 6, 9, 11, 29
- Metainfo Extensions: BEP 12, 17, 19, 27, 47

## Test Vectors (Section B)
- Bencode examples for integers, byte strings, lists, dictionaries
- Hex representations and decoded values
- Invalid encoding examples (negative zero, leading zeros, etc.)
- Info hash examples for known public torrents (Big Buck Bunny, Ubuntu ISO)
- Verification procedure and common mistakes

## Future Work (Section C)
- DHT: Kademlia algorithm, implementation requirements, benefits
- uTP: LEDBAT congestion control, network friendliness
- PEX: Peer exchange protocol details
- Magnet Links: URI format, download flow
- Other extensions: BEP 6, 14, 40, 24
- Recommended extension order for further development

## Additional Sections
- Glossary: 25 BitTorrent-specific terms defined
- References: Primary (BEP URLs) and secondary sources (wiki, papers)

Build verification:
- just tangle: Success
- just weave: Success (687-page PDF generated)
<!-- SECTION:NOTES:END -->
