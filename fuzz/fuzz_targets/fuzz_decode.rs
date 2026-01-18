//! Fuzz target for bencode decoder with arbitrary bytes.
//!
//! This target feeds arbitrary byte sequences to the bencode decoder to find:
//! - Panics on malformed input
//! - Memory safety issues
//! - Hangs or excessive resource consumption
//!
//! The decoder should gracefully return an error for any invalid input,
//! never panic or crash.

#![no_main]

use libfuzzer_sys::fuzz_target;
use literate_bittorrent::bencode::decode;

fuzz_target!(|data: &[u8]| {
    // The decoder must not panic on any input.
    // It's expected to return Ok for valid bencode or Err for invalid input.
    // We don't care about the result, only that it doesn't crash.
    let _ = decode(data);
});
