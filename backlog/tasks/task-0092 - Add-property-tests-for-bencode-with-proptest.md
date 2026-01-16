---
id: task-0092
title: Add property tests for bencode with proptest
status: To Do
assignee: []
created_date: '2026-01-16 21:33'
labels:
  - bencode
  - testing
  - proptest
dependencies:
  - task-0085
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Property-based testing validates encode/decode inverse property across randomly generated inputs. This catches edge cases that manual tests miss. Critical for confidence in the bencode implementation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 proptest generates arbitrary BencodeValue instances
- [ ] #2 Property: for all v, decode(encode(v)) == Ok(v)
- [ ] #3 Property: for all valid encoded bytes b, encode(decode(b)) == Ok(b)
- [ ] #4 Shrinking works to find minimal failing cases
- [ ] #5 Tests run with cargo test --features proptest or similar
- [ ] #6 At least 1000 test cases pass
<!-- AC:END -->
