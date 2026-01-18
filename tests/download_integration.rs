// Integration test for downloading a complete piece from a real BitTorrent swarm.
//
// This test proves the BitTorrent client can actually download data from real peers.
// It downloads at least one piece, verifies its SHA1 hash, and writes to disk.
//
// Run with: cargo test --test download_integration -- --ignored --nocapture
// Or via justfile: just e2e
//
// TEST TORRENT DOCUMENTATION:
// ===========================
// Torrent: ubuntu-24.04.3-live-server-amd64.iso
// Info hash: a1dfefec1a9dd7fa8a041ebeeea271db55126d2f
// Total size: 3,303,444,480 bytes (3.3 GB)
// Piece length: 262,144 bytes (256 KiB)
// Number of pieces: 12,602
//
// Why this torrent:
// - Ubuntu torrents are well-seeded (100+ seeders typically)
// - Legal, publicly available
// - Large enough that partial download is meaningful
// - Stable metadata (won't disappear)
//
// We download piece 0 (first piece) as it's most likely to be available.
// Expected SHA1 of piece 0: from torrent metadata

use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use literate_bittorrent::client::{
    connect_with_timeout, perform_handshake, read_message, write_message,
};
use literate_bittorrent::disk::{create_output_files, write_piece};
use literate_bittorrent::messages::{Handshake, Message, PeerError};
use literate_bittorrent::metainfo::{parse, FileInfo, Metainfo};
use literate_bittorrent::pieces::{verify_piece, InProgressPiece, BLOCK_SIZE};
use literate_bittorrent::tracker::{
    announce, generate_peer_id, TrackerEvent, TrackerRequest,
};
use tempfile::tempdir;
use tokio::time::timeout;

/// Overall test timeout in seconds.
/// The test must complete all steps within this time.
const TEST_TIMEOUT_SECS: u64 = 300; // 5 minutes

/// Maximum time to wait for unchoke message after sending interested.
const UNCHOKE_TIMEOUT_SECS: u64 = 30;

/// Maximum peers to try before giving up.
const MAX_PEERS_TO_TRY: usize = 15;

/// Maximum time to wait for a single block response.
const BLOCK_TIMEOUT_SECS: u64 = 30;

/// Convert a 20-byte hash to a hex string
fn hash_to_hex(hash: &[u8; 20]) -> String {
    hash.iter().fold(String::with_capacity(40), |mut s, b| {
        let _ = write!(s, "{:02x}", b);
        s
    })
}

/// Path to the test fixture torrent file
fn fixture_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/ubuntu.torrent");
    path
}

/// Load and parse the ubuntu.torrent fixture
fn load_ubuntu_torrent() -> Metainfo {
    let path = fixture_path();
    let data = fs::read(&path).expect("Failed to read ubuntu.torrent fixture");
    parse(&data).expect("Failed to parse ubuntu.torrent")
}

/// Create a TrackerRequest for the initial announce
fn create_announce_request(metainfo: &Metainfo, peer_id: [u8; 20]) -> TrackerRequest {
    TrackerRequest {
        info_hash: metainfo.info_hash,
        peer_id,
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: metainfo.total_length,
        event: Some(TrackerEvent::Started),
        compact: true,
    }
}

/// Result of attempting to download a piece from a peer.
enum DownloadResult {
    /// Successfully downloaded and verified a piece.
    Success {
        peer_addr: SocketAddr,
        piece_index: u32,
        piece_data: Vec<u8>,
    },
    /// Failed at some stage.
    Failed {
        peer_addr: SocketAddr,
        error: String,
    },
}

/// Select a piece to download from peer's bitfield.
///
/// Returns the index of a piece the peer has, preferring earlier pieces.
fn select_piece_from_bitfield(bitfield: &[u8], num_pieces: u32) -> Option<u32> {
    for (byte_idx, byte) in bitfield.iter().enumerate() {
        for bit_idx in 0..8 {
            let piece_idx = (byte_idx * 8 + bit_idx) as u32;
            if piece_idx >= num_pieces {
                return None;
            }
            // BitTorrent uses MSB-first ordering
            if byte & (1 << (7 - bit_idx)) != 0 {
                return Some(piece_idx);
            }
        }
    }
    None
}

/// Calculate the actual size of a piece (last piece may be smaller).
fn calculate_piece_size(piece_index: u32, piece_length: u32, total_length: u64, num_pieces: u32) -> u32 {
    if piece_index == num_pieces - 1 {
        // Last piece may be smaller
        let full_pieces_bytes = (num_pieces - 1) as u64 * piece_length as u64;
        (total_length - full_pieces_bytes) as u32
    } else {
        piece_length
    }
}

/// Try to download one piece from a single peer.
async fn try_download_piece_from_peer(
    peer_addr: SocketAddr,
    our_handshake: &Handshake,
    metainfo: &Metainfo,
) -> DownloadResult {
    println!("  Trying peer {}...", peer_addr);

    // Step 1: Connect with timeout
    let mut stream = match connect_with_timeout(peer_addr).await {
        Ok(s) => s,
        Err(e) => {
            return DownloadResult::Failed {
                peer_addr,
                error: format!("connect failed: {}", e),
            };
        }
    };
    println!("    Connected");

    // Step 2: Perform handshake
    let _peer_handshake = match perform_handshake(&mut stream, our_handshake, &metainfo.info_hash).await {
        Ok(h) => h,
        Err(e) => {
            return DownloadResult::Failed {
                peer_addr,
                error: format!("handshake failed: {}", e),
            };
        }
    };
    println!("    Handshake OK");

    // Step 3: Wait for bitfield or have messages, track available pieces
    let mut available_pieces: HashSet<u32> = HashSet::new();
    let mut received_unchoke = false;
    let bitfield_deadline = tokio::time::Instant::now() + Duration::from_secs(10);

    while tokio::time::Instant::now() < bitfield_deadline {
        let msg = match timeout(Duration::from_secs(5), read_message(&mut stream)).await {
            Ok(Ok(msg)) => msg,
            Ok(Err(PeerError::ConnectionClosed)) => {
                return DownloadResult::Failed {
                    peer_addr,
                    error: "connection closed while waiting for bitfield".to_string(),
                };
            }
            Ok(Err(e)) => {
                return DownloadResult::Failed {
                    peer_addr,
                    error: format!("read error: {}", e),
                };
            }
            Err(_) => continue, // Timeout, continue loop
        };

        match msg {
            Message::Bitfield(bf) => {
                println!("    Received Bitfield ({} bytes)", bf.len());
                // Parse bitfield to find available pieces
                if let Some(piece_idx) = select_piece_from_bitfield(&bf, metainfo.piece_hashes.len() as u32) {
                    println!("    Peer has piece {}", piece_idx);
                    available_pieces.insert(piece_idx);
                    // Also mark other pieces as available
                    for (byte_idx, byte) in bf.iter().enumerate() {
                        for bit_idx in 0..8 {
                            let idx = (byte_idx * 8 + bit_idx) as u32;
                            if idx < metainfo.piece_hashes.len() as u32 && byte & (1 << (7 - bit_idx)) != 0 {
                                available_pieces.insert(idx);
                            }
                        }
                    }
                }
                break;
            }
            Message::Have(piece) => {
                println!("    Received Have({})", piece);
                available_pieces.insert(piece);
            }
            Message::Unchoke => {
                println!("    Received Unchoke (early)");
                received_unchoke = true;
            }
            Message::Choke => {
                println!("    Received Choke");
            }
            Message::KeepAlive => {}
            _ => {}
        }
    }

    if available_pieces.is_empty() {
        return DownloadResult::Failed {
            peer_addr,
            error: "peer has no pieces available".to_string(),
        };
    }

    // Select piece 0 if available, otherwise any available piece
    let piece_to_download = if available_pieces.contains(&0) {
        0
    } else {
        *available_pieces.iter().min().unwrap()
    };
    println!("    Will download piece {}", piece_to_download);

    // Step 4: Send Interested message
    if let Err(e) = write_message(&mut stream, &Message::Interested).await {
        return DownloadResult::Failed {
            peer_addr,
            error: format!("failed to send Interested: {}", e),
        };
    }
    println!("    Sent Interested");

    // Step 5: Wait for Unchoke if not already received
    if !received_unchoke {
        let unchoke_deadline = tokio::time::Instant::now() + Duration::from_secs(UNCHOKE_TIMEOUT_SECS);

        while tokio::time::Instant::now() < unchoke_deadline && !received_unchoke {
            let remaining = unchoke_deadline - tokio::time::Instant::now();
            let msg = match timeout(remaining, read_message(&mut stream)).await {
                Ok(Ok(msg)) => msg,
                Ok(Err(e)) => {
                    return DownloadResult::Failed {
                        peer_addr,
                        error: format!("read error waiting for unchoke: {}", e),
                    };
                }
                Err(_) => {
                    return DownloadResult::Failed {
                        peer_addr,
                        error: "timeout waiting for unchoke".to_string(),
                    };
                }
            };

            match msg {
                Message::Unchoke => {
                    println!("    Received Unchoke");
                    received_unchoke = true;
                    break;
                }
                Message::Have(piece) => {
                    available_pieces.insert(piece);
                }
                Message::KeepAlive | Message::Choke => {}
                _ => {}
            }
        }

        if !received_unchoke {
            return DownloadResult::Failed {
                peer_addr,
                error: "did not receive unchoke".to_string(),
            };
        }
    }

    // Step 6: Download the piece block by block
    let num_pieces = metainfo.piece_hashes.len() as u32;
    let piece_size = calculate_piece_size(
        piece_to_download,
        metainfo.piece_length,
        metainfo.total_length,
        num_pieces,
    );
    println!("    Piece size: {} bytes", piece_size);

    let mut in_progress = InProgressPiece::new(piece_to_download, piece_size);
    let num_blocks = in_progress.num_blocks();
    println!("    Blocks to download: {}", num_blocks);

    // Send all block requests
    for block_idx in 0..num_blocks {
        let offset = block_idx * BLOCK_SIZE;
        let length = if block_idx == num_blocks - 1 {
            piece_size - offset
        } else {
            BLOCK_SIZE
        };

        let request = Message::Request {
            index: piece_to_download,
            begin: offset,
            length,
        };

        if let Err(e) = write_message(&mut stream, &request).await {
            return DownloadResult::Failed {
                peer_addr,
                error: format!("failed to send request for block {}: {}", block_idx, e),
            };
        }
    }
    println!("    Sent {} block requests", num_blocks);

    // Receive all blocks
    let download_deadline = tokio::time::Instant::now() + Duration::from_secs(60);
    let mut blocks_received = 0;

    while !in_progress.is_complete() && tokio::time::Instant::now() < download_deadline {
        let msg = match timeout(
            Duration::from_secs(BLOCK_TIMEOUT_SECS),
            read_message(&mut stream),
        )
        .await
        {
            Ok(Ok(msg)) => msg,
            Ok(Err(e)) => {
                return DownloadResult::Failed {
                    peer_addr,
                    error: format!("read error waiting for piece data: {}", e),
                };
            }
            Err(_) => {
                return DownloadResult::Failed {
                    peer_addr,
                    error: format!(
                        "timeout waiting for block (received {}/{})",
                        blocks_received, num_blocks
                    ),
                };
            }
        };

        match msg {
            Message::Piece { index, begin, data } => {
                if index == piece_to_download {
                    match in_progress.receive_block(begin, &data) {
                        Ok(true) => {
                            blocks_received += 1;
                            println!(
                                "    Received block {} ({}/{} blocks)",
                                begin / BLOCK_SIZE,
                                blocks_received,
                                num_blocks
                            );
                        }
                        Ok(false) => {
                            println!("    Duplicate block at offset {}", begin);
                        }
                        Err(e) => {
                            return DownloadResult::Failed {
                                peer_addr,
                                error: format!("invalid block: {}", e),
                            };
                        }
                    }
                }
            }
            Message::Choke => {
                return DownloadResult::Failed {
                    peer_addr,
                    error: "peer choked us during download".to_string(),
                };
            }
            Message::KeepAlive | Message::Have(_) => {}
            _ => {}
        }
    }

    if !in_progress.is_complete() {
        return DownloadResult::Failed {
            peer_addr,
            error: format!(
                "download incomplete: received {}/{} blocks",
                blocks_received, num_blocks
            ),
        };
    }

    println!("    Download complete, verifying hash...");

    // Step 7: Verify piece hash
    let expected_hash = &metainfo.piece_hashes[piece_to_download as usize];
    let piece_data = in_progress.data().to_vec();

    match verify_piece(piece_to_download, &piece_data, expected_hash) {
        Ok(()) => {
            println!(
                "    Hash verified! Expected: {}",
                hash_to_hex(expected_hash)
            );
            DownloadResult::Success {
                peer_addr,
                piece_index: piece_to_download,
                piece_data,
            }
        }
        Err(e) => DownloadResult::Failed {
            peer_addr,
            error: format!("hash verification failed: {}", e),
        },
    }
}

/// Integration test: download and verify a complete piece from a real BitTorrent swarm.
///
/// This test:
/// 1. Loads the ubuntu.torrent fixture
/// 2. Announces to the tracker to get a peer list
/// 3. Tries connecting to peers until one successfully responds
/// 4. Downloads a complete piece (all blocks)
/// 5. Verifies the piece hash matches expected (from torrent metadata)
/// 6. Writes the piece to disk
/// 7. Reads back and verifies the written data
///
/// Requirements:
/// - Network access to tracker and peers
/// - At least one responsive, unchoking peer in the swarm
///
/// Note: This test is marked #[ignore] because it requires network access
/// and depends on external peers. Run explicitly with:
///   cargo test --test download_integration -- --ignored --nocapture
///   just e2e
#[tokio::test]
#[ignore]
async fn test_download_piece_from_swarm() {
    // Initialize tracing for debug output
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init();

    // Wrap entire test in timeout
    let test_result = timeout(Duration::from_secs(TEST_TIMEOUT_SECS), async {
        println!("\n=== Download Integration Test ===");
        println!("This test downloads and verifies a complete piece from a real BitTorrent swarm.\n");

        // Step 1: Load torrent
        println!("Step 1: Loading ubuntu.torrent...");
        let metainfo = load_ubuntu_torrent();
        println!("  Torrent: {}", metainfo.name);
        println!("  Info hash: {}", hash_to_hex(&metainfo.info_hash));
        println!("  Total size: {} bytes", metainfo.total_length);
        println!("  Piece length: {} bytes", metainfo.piece_length);
        println!("  Number of pieces: {}", metainfo.piece_hashes.len());

        // Step 2: Generate peer ID and announce to tracker
        let our_peer_id = generate_peer_id();
        println!("\nStep 2: Announcing to tracker...");
        println!("  Tracker URL: {}", metainfo.announce);

        let request = create_announce_request(&metainfo, our_peer_id);
        let announce_result = match announce(&metainfo, request).await {
            Ok(r) => r,
            Err(e) => {
                panic!(
                    "Tracker announce failed: {}. This may be a transient network error.",
                    e
                );
            }
        };

        println!("  Tracker returned {} peers", announce_result.peers.len());
        if let Some(seeders) = announce_result.seeders {
            println!("  Seeders: {}", seeders);
        }

        if announce_result.peers.is_empty() {
            panic!("Tracker returned no peers. The swarm may be inactive.");
        }

        // Step 3: Create handshake and try peers
        println!("\nStep 3: Connecting to peers and downloading...");
        let our_handshake = Handshake::new(metainfo.info_hash, our_peer_id);

        let mut success_result: Option<(SocketAddr, u32, Vec<u8>)> = None;
        let mut failures: Vec<(SocketAddr, String)> = Vec::new();

        let peers_to_try = announce_result.peers.len().min(MAX_PEERS_TO_TRY);
        for peer_addr in announce_result.peers.iter().take(peers_to_try) {
            match try_download_piece_from_peer(*peer_addr, &our_handshake, &metainfo).await {
                DownloadResult::Success {
                    peer_addr,
                    piece_index,
                    piece_data,
                } => {
                    success_result = Some((peer_addr, piece_index, piece_data));
                    break;
                }
                DownloadResult::Failed { peer_addr, error } => {
                    println!("  Peer {} failed: {}", peer_addr, error);
                    failures.push((peer_addr, error));
                }
            }
        }

        let (peer_addr, piece_index, piece_data) = match success_result {
            Some(r) => r,
            None => {
                panic!(
                    "Failed to download from any peer. Tried {} peers:\n{}",
                    failures.len(),
                    failures
                        .iter()
                        .map(|(addr, err)| format!("  {}: {}", addr, err))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        };

        println!("\n  SUCCESS: Downloaded piece {} from {}", piece_index, peer_addr);
        println!("  Piece size: {} bytes", piece_data.len());

        // Step 4: Write piece to disk
        println!("\nStep 4: Writing piece to disk...");
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let output_dir = temp_dir.path();
        println!("  Output directory: {:?}", output_dir);

        // Create output files (we only need the first file for piece 0)
        let files: Vec<FileInfo> = vec![FileInfo {
            length: metainfo.total_length,
            path: PathBuf::from(&metainfo.name),
        }];

        create_output_files(output_dir, &files).expect("Failed to create output files");

        write_piece(
            piece_index,
            &piece_data,
            metainfo.piece_length,
            metainfo.total_length,
            output_dir,
            &files,
        )
        .expect("Failed to write piece to disk");

        println!("  Piece written successfully");

        // Step 5: Read back and verify
        println!("\nStep 5: Verifying written data...");
        let file_path = output_dir.join(&metainfo.name);

        // Read the piece portion back from the file
        let piece_offset = piece_index as u64 * metainfo.piece_length as u64;
        let file_content = fs::read(&file_path).expect("Failed to read output file");
        let read_back = &file_content[piece_offset as usize..piece_offset as usize + piece_data.len()];

        assert_eq!(
            read_back, &piece_data[..],
            "Read-back data does not match downloaded data"
        );
        println!("  Disk verification passed: written data matches downloaded data");

        // Step 6: Verify hash of read-back data
        let expected_hash = &metainfo.piece_hashes[piece_index as usize];
        verify_piece(piece_index, read_back, expected_hash)
            .expect("Read-back data hash verification failed");
        println!("  Hash verification passed: {}", hash_to_hex(expected_hash));

        // Cleanup happens automatically when temp_dir goes out of scope
        println!("\nStep 6: Cleanup (automatic via tempfile)");

        // Report results
        println!("\n=== Test Results ===");
        println!("Peers tried: {}", failures.len() + 1);
        println!("Successful peer: {}", peer_addr);
        println!("Piece downloaded: {} ({} bytes)", piece_index, piece_data.len());
        println!("Hash verified: {}", hash_to_hex(expected_hash));
        println!("Disk write verified: YES");
        println!("\nTest PASSED: Successfully downloaded, verified, and wrote piece to disk.");
        println!("=== End of Test ===\n");
    })
    .await;

    match test_result {
        Ok(_) => {
            // Test completed within timeout
        }
        Err(_) => {
            panic!(
                "Test timed out after {} seconds ({} minutes). \
                 This may indicate network issues or unresponsive peers.",
                TEST_TIMEOUT_SECS,
                TEST_TIMEOUT_SECS / 60
            );
        }
    }
}

/// Test that verifies the download integration test setup without network access.
/// This test does NOT require network access.
#[test]
fn test_download_integration_setup() {
    // Verify fixture loads
    let metainfo = load_ubuntu_torrent();
    assert!(!metainfo.announce.is_empty());
    assert_eq!(metainfo.info_hash.len(), 20);
    assert!(!metainfo.piece_hashes.is_empty());

    // Verify piece size calculation
    let num_pieces = metainfo.piece_hashes.len() as u32;
    let piece_0_size = calculate_piece_size(0, metainfo.piece_length, metainfo.total_length, num_pieces);
    assert_eq!(piece_0_size, metainfo.piece_length);

    // Last piece should be smaller (unless perfect alignment)
    let last_piece_size = calculate_piece_size(
        num_pieces - 1,
        metainfo.piece_length,
        metainfo.total_length,
        num_pieces,
    );
    assert!(last_piece_size <= metainfo.piece_length);

    // Verify InProgressPiece can be created
    let piece = InProgressPiece::new(0, piece_0_size);
    assert_eq!(piece.piece_index(), 0);
    assert_eq!(piece.piece_size(), piece_0_size);
    assert!(!piece.is_complete());

    // Verify bitfield parsing
    let bitfield = vec![0b10000000u8]; // First piece available
    assert_eq!(select_piece_from_bitfield(&bitfield, 8), Some(0));

    let bitfield2 = vec![0b01000000u8]; // Second piece available
    assert_eq!(select_piece_from_bitfield(&bitfield2, 8), Some(1));

    let bitfield_empty = vec![0b00000000u8]; // No pieces available
    assert_eq!(select_piece_from_bitfield(&bitfield_empty, 8), None);

    println!("Download integration setup test passed");
}
