---
id: task-0078
title: Implement bencode encoder
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:31'
updated_date: '2026-01-16 23:54'
labels:
  - bencode
  - encoder
dependencies:
  - task-0071
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Encoder is necessary for info hash computation - we must re-encode the info dict to compute its SHA1. Encoding is simpler than decoding as we just traverse the BencodeValue tree. BTreeMap ensures canonical dict key ordering.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Encodes Integer correctly (42 -> i42e)
- [x] #2 Encodes negative integers and zero
- [x] #3 Encodes Bytes correctly (vec -> length:bytes)
- [x] #4 Encodes empty byte strings
- [x] #5 Encodes List recursively
- [x] #6 Encodes Dict with keys in sorted byte order (BTreeMap handles this)
- [x] #7 Encodes nested structures correctly
- [x] #8 Unit tests cover all types
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add prose section explaining encoding (inverse of decoding)
2. Implement encode_to(value: &BencodeValue, out: &mut Vec<u8>) for efficiency
3. Implement encode(value: &BencodeValue) -> Vec<u8> wrapper
4. Add comprehensive tests for all types:
   - Integer encoding (positive, negative, zero)
   - Byte string encoding (including empty)
   - List encoding (including nested)
   - Dict encoding (verify sorted key order)
   - Nested structures
5. Verify round-trip: decode(encode(x)) == x
6. Update module structure to include encode chunk
7. Run tests to verify implementation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented bencode encoder with:

- `encode_to(value: &BencodeValue, out: &mut Vec<u8>)` - efficient encoder that appends to existing buffer
- `encode(value: &BencodeValue) -> Vec<u8>` - convenience wrapper that allocates new buffer

Added comprehensive test coverage:
- Integer encoding: positive, negative, zero, i64::MAX, i64::MIN
- Bytes encoding: simple strings, empty, binary data with non-UTF8 bytes
- List encoding: empty, single element, multiple elements, nested lists
- Dict encoding: empty, single entry, multiple entries (verifies sorted key order), nested dicts
- Round-trip tests: decode(encode(x)) == x for all types including complex nested structures

BTreeMap ensures canonical key ordering for dictionaries, which is critical for info hash computation.
<!-- SECTION:NOTES:END -->
