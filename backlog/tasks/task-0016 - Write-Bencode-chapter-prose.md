---
id: task-0016
title: Write Bencode chapter prose
status: To Do
assignee: []
created_date: '2026-01-16 21:27'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bencode is the foundational data format used throughout BitTorrent. The chapter must explain WHY a custom serialization format exists (simplicity, canonical encoding for hashing), define the grammar clearly, walk through the decoder implementation line-by-line explaining design choices, then cover the encoder. The round-trip property (encode(decode(x)) == x) should be emphasized as it is crucial for info hash computation. Code walkthrough should build understanding incrementally - integers first, then strings, then collections.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Explains why BitTorrent uses Bencode instead of JSON/XML (canonical encoding, simplicity)
- [ ] #2 Formal grammar definition is provided and explained
- [ ] #3 Decoder implementation walkthrough explains each parsing step and error cases
- [ ] #4 Encoder implementation shows symmetric structure to decoder
- [ ] #5 Round-trip property is demonstrated and its importance for hashing explained
<!-- AC:END -->
