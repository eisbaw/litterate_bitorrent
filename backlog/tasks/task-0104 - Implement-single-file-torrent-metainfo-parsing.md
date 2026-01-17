---
id: task-0104
title: Implement single-file torrent metainfo parsing
status: Done
assignee:
  - '@claude'
created_date: '2026-01-16 21:34'
updated_date: '2026-01-17 00:12'
labels:
  - metainfo
  - parser
dependencies:
  - task-0102
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parse .torrent files in single-file format. Extract announce URL, name, piece length, and piece hashes from bencoded dict. Single-file format has length field directly in info dict rather than files array.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Parses announce URL from top-level dict
- [x] #2 Extracts name from info dict
- [x] #3 Extracts piece_length from info dict
- [x] #4 Parses pieces byte string into Vec of 20-byte SHA1 hashes
- [x] #5 Returns clear error if pieces length is not multiple of 20
- [x] #6 Extracts length field for single-file format
- [x] #7 Creates single FileInfo entry with name and length
- [x] #8 Computes total_length correctly
- [x] #9 Unit test parses hand-crafted minimal single-file torrent
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add helper functions to extract values from BencodeValue:
   - get_dict: Extract BTreeMap from BencodeValue::Dict
   - get_bytes: Extract Vec<u8> from BencodeValue::Bytes
   - get_string: Extract String from BencodeValue::Bytes (UTF-8)
   - get_int: Extract i64 from BencodeValue::Integer

2. Add parse_pieces function to split pieces bytes into 20-byte hashes

3. Add parse_single_file function to extract FileInfo from info dict

4. Add parse function as main entry point that:
   - Decodes raw bytes using crate::bencode::decode
   - Extracts announce URL from top-level dict
   - Extracts info dict
   - Parses piece_length, pieces, name from info dict
   - Detects single-file vs multi-file format
   - Uses placeholder [0u8; 20] for info_hash (separate task 0110)

5. Add unit tests for single-file torrent parsing with hand-crafted data
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented single-file torrent metainfo parsing in nw/02-metainfo.nw.

Changes:
- Added helper functions for extracting values from BencodeValue:
  - get_dict: Extract dictionary from value
  - get_bytes: Extract byte string by key
  - get_string: Extract UTF-8 string by key
  - get_int: Extract integer by key
  - get_dict_field: Extract nested dictionary by key

- Added parse_pieces function to split pieces bytes into Vec<[u8; 20]>
  - Validates length is multiple of 20
  - Returns InvalidPieceHashes error for invalid length

- Added parse_single_file function for single-file format
  - Extracts length from info dict
  - Creates single FileInfo with name as path

- Added parse function as main entry point
  - Decodes bencode data
  - Extracts announce URL from top-level dict
  - Extracts info dict and parses name, piece_length, pieces
  - Validates piece_length is positive and within u32 range
  - Uses placeholder [0u8; 20] for info_hash (task 0110)
  - Multi-file format returns clear error (task 0106)

- Added comprehensive unit tests:
  - parse_minimal_single_file_torrent: Happy path with all fields verified
  - parse_torrent_with_multiple_pieces: Multiple piece hashes
  - reject_invalid_pieces_length: Not multiple of 20
  - reject_zero_piece_length / reject_negative_piece_length
  - reject_missing_* tests for all required fields
  - reject_empty_bencode / reject_non_dict_root
  - parse_pieces_empty / parse_pieces_multiple

All 138 tests pass. Clippy passes with no warnings.
<!-- SECTION:NOTES:END -->
