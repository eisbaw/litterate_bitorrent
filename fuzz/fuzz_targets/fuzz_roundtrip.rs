//! Fuzz target for bencode encode-decode roundtrip.
//!
//! This target generates structured BencodeValue instances, encodes them,
//! then decodes the result and verifies the roundtrip produces the original value.
//!
//! This catches:
//! - Encoder bugs that produce invalid bencode
//! - Decoder bugs that misparse valid bencode
//! - Inconsistencies between encoder and decoder

#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use literate_bittorrent::bencode::{decode, encode, BencodeValue};
use std::collections::BTreeMap;

/// Wrapper type that implements Arbitrary for BencodeValue.
///
/// We limit recursion depth and sizes to prevent the fuzzer from generating
/// extremely deep or large structures that would slow down fuzzing.
#[derive(Debug, Clone)]
struct ArbitraryBencode(BencodeValue);

impl<'a> Arbitrary<'a> for ArbitraryBencode {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(ArbitraryBencode(arbitrary_bencode_value(u, 4)?))
    }
}

/// Generate an arbitrary BencodeValue with bounded depth.
fn arbitrary_bencode_value(u: &mut Unstructured, depth: usize) -> arbitrary::Result<BencodeValue> {
    if depth == 0 {
        // At max depth, only generate leaf values (integers or bytes)
        return if u.arbitrary()? {
            Ok(BencodeValue::Integer(u.arbitrary()?))
        } else {
            let bytes: Vec<u8> = arbitrary_bounded_bytes(u, 64)?;
            Ok(BencodeValue::Bytes(bytes))
        };
    }

    // Choose a type: 0=integer, 1=bytes, 2=list, 3=dict
    let choice: u8 = u.int_in_range(0..=3)?;

    match choice {
        0 => Ok(BencodeValue::Integer(u.arbitrary()?)),
        1 => {
            let bytes = arbitrary_bounded_bytes(u, 64)?;
            Ok(BencodeValue::Bytes(bytes))
        }
        2 => {
            // List with 0-8 elements
            let len: usize = u.int_in_range(0..=8)?;
            let mut list = Vec::with_capacity(len);
            for _ in 0..len {
                list.push(arbitrary_bencode_value(u, depth - 1)?);
            }
            Ok(BencodeValue::List(list))
        }
        3 => {
            // Dict with 0-8 entries
            let len: usize = u.int_in_range(0..=8)?;
            let mut dict = BTreeMap::new();
            for _ in 0..len {
                let key = arbitrary_bounded_bytes(u, 32)?;
                let value = arbitrary_bencode_value(u, depth - 1)?;
                dict.insert(key, value);
            }
            Ok(BencodeValue::Dict(dict))
        }
        _ => unreachable!(),
    }
}

/// Generate a byte vector with bounded length.
fn arbitrary_bounded_bytes(u: &mut Unstructured, max_len: usize) -> arbitrary::Result<Vec<u8>> {
    let len: usize = u.int_in_range(0..=max_len)?;
    let mut bytes = vec![0u8; len];
    u.fill_buffer(&mut bytes)?;
    Ok(bytes)
}

fuzz_target!(|data: ArbitraryBencode| {
    let original = data.0;

    // Encode the value
    let encoded = encode(&original);

    // Decode should succeed and return the original value
    match decode(&encoded) {
        Ok(decoded) => {
            assert_eq!(
                decoded, original,
                "Roundtrip failed: encode then decode produced different value"
            );

            // Also verify that re-encoding produces the same bytes (canonical encoding)
            let re_encoded = encode(&decoded);
            assert_eq!(
                re_encoded, encoded,
                "Non-canonical encoding: re-encoding produced different bytes"
            );
        }
        Err(e) => {
            panic!(
                "Decoder failed on encoder output: {:?}\nEncoded bytes: {:?}",
                e, encoded
            );
        }
    }
});
