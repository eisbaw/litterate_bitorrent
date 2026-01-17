---
id: task-0092
title: Add property tests for bencode with proptest
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:33'
updated_date: '2026-01-17 00:02'
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
- [x] #1 proptest generates arbitrary BencodeValue instances
- [x] #2 Property: for all v, decode(encode(v)) == Ok(v)
- [x] #3 Property: for all valid encoded bytes b, encode(decode(b)) == Ok(b)
- [x] #4 Shrinking works to find minimal failing cases
- [x] #5 Tests run with cargo test --features proptest or similar
- [x] #6 At least 1000 test cases pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze the BencodeValue type and understand what valid values look like
2. Create arb_bencode_value() strategy using prop_oneof\! and prop_recursive
3. Add proptest_roundtrip_encode_decode: decode(encode(v)) == Ok(v)
4. Add proptest_roundtrip_decode_encode: for valid encoded bytes, encode(decode(b)) == Ok(b)
5. Configure proptest to run 1000+ cases with PROPTEST_CASES=1000
6. Verify shrinking works by temporarily introducing a bug
7. Run just test to ensure all tests pass
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added property-based tests for bencode using proptest crate.

Changes made to nw/01-bencode.nw:
- Added new proptest section with LaTeX documentation explaining property-based testing
- Created arb_bencode_value() strategy using prop_recursive for nested structures
- Added proptest_roundtrip_encode_decode test: verifies decode(encode(v)) == Ok(v)
- Added proptest_roundtrip_decode_encode test: verifies canonical encoding
- Configured proptest to run 1000 test cases per property

Verified:
- All 124 tests pass (122 unit tests + 2 property tests)
- Shrinking works correctly (demonstrated with temporary failing test)
- Clippy linting passes with no warnings
- Tests run via just test command
<!-- SECTION:NOTES:END -->
