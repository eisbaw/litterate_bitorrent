---
id: task-0061
title: Implement bencode dict decoder
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:29'
updated_date: '2026-01-16 23:44'
labels:
  - bencode
  - parser
dependencies:
  - task-0049
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Dictionaries are encoded as d<key><value>...e where keys must be byte strings in sorted order. BTreeMap ensures canonical ordering for info hash computation. This completes the recursive descent decoder.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Decodes empty dict (de -> {})
- [x] #2 Decodes dict with single key-value pair
- [x] #3 Decodes dict with multiple entries
- [x] #4 Decodes nested dicts (dict containing dict)
- [x] #5 Rejects non-string keys with clear error
- [x] #6 Handles keys with different sorting than ASCII order (raw byte comparison)
- [x] #7 Returns error for unclosed dict
- [x] #8 Returns error for duplicate keys
- [x] #9 Unit tests cover all cases
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add DuplicateKey error variant to BencodeError enum
2. Implement decode_dict function chunk following the pattern of decode_list
3. Update decode_value dispatcher to call decode_dict for 'd' marker
4. Add comprehensive unit tests for all acceptance criteria
5. Update bencode.rs module structure to include decode_dict chunk
6. Run just tangle && just test to verify
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented bencode dictionary decoder completing the recursive descent parser.

Changes:
- Added DuplicateKey and InvalidDictKey error variants to BencodeError
- Implemented decode_dict function following the same pattern as decode_list
- Updated decode_value dispatcher to route 'd' to decode_dict
- Added 26 comprehensive tests covering all acceptance criteria

Key design decisions:
- Use BTreeMap<Vec<u8>, BencodeValue> for sorted key ordering (critical for info hash)
- Explicitly check for non-string keys before calling decode_bytes for cleaner errors
- Accept out-of-order keys (BTreeMap sorts them) but reject duplicate keys
- Binary keys sort correctly by raw byte value

All 78 tests pass including the new dictionary tests.
<!-- SECTION:NOTES:END -->
