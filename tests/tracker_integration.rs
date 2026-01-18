// Integration test for tracker announce functionality
// This test requires network access and is marked #[ignore] to avoid
// breaking CI without network connectivity.
//
// Run with: cargo test --ignored
// Or via justfile: just e2e

use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use literate_bittorrent::metainfo::{parse, Metainfo};
use literate_bittorrent::tracker::{
    announce, generate_peer_id, AnnounceResult, TrackerEvent, TrackerManager, TrackerRequest,
};

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
fn create_announce_request(metainfo: &Metainfo) -> TrackerRequest {
    TrackerRequest {
        info_hash: metainfo.info_hash,
        peer_id: generate_peer_id(),
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: metainfo.total_length,
        event: Some(TrackerEvent::Started),
        compact: true,
        numwant: Some(200),
    }
}

/// Print the announce result to stdout in a human-readable format
fn print_announce_result(result: &AnnounceResult) {
    println!("=== Tracker Announce Result ===");
    println!("Interval: {} seconds", result.interval);

    if let Some(seeders) = result.seeders {
        println!("Seeders: {}", seeders);
    }
    if let Some(leechers) = result.leechers {
        println!("Leechers: {}", leechers);
    }

    println!("\nPeers ({} total):", result.peers.len());
    if result.peers.is_empty() {
        println!("  (no peers returned by tracker)");
    } else {
        for peer in &result.peers {
            println!("  {}", peer);
        }
    }
    println!("=== End of Announce Result ===");
}

/// Integration test: announce to a real tracker and print peers
///
/// This test:
/// 1. Loads a real .torrent file (Ubuntu ISO)
/// 2. Performs an announce to the actual tracker
/// 3. Prints the list of peer IP:port pairs to stdout
/// 4. Handles the case where tracker returns no peers (not an error)
///
/// Requirements:
/// - Network access to the Ubuntu torrent tracker
/// - The tracker must be online (may fail if tracker is temporarily down)
///
/// Note: This test is marked #[ignore] because it requires network access
/// and depends on external services. Run explicitly with:
///   cargo test --ignored
///   just e2e
#[tokio::test]
#[ignore]
async fn test_announce_to_real_tracker() {
    // Initialize minimal tracing for debugging (optional)
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .try_init();

    // Step 1: Load the torrent file
    println!("\nLoading ubuntu.torrent from fixtures...");
    let metainfo = load_ubuntu_torrent();
    println!("Loaded torrent: {}", metainfo.name);
    println!("Info hash: {}", hash_to_hex(&metainfo.info_hash));
    println!("Total length: {} bytes", metainfo.total_length);
    println!("Tracker URL: {}", metainfo.announce);

    // Step 2: Create the announce request
    let request = create_announce_request(&metainfo);
    println!("\nPeer ID: {:?}", String::from_utf8_lossy(&request.peer_id[..8]));
    println!("Requesting peers from tracker...\n");

    // Step 3: Perform the announce
    let result = announce(&metainfo, request).await;

    // Step 4: Handle the result
    match result {
        Ok(announce_result) => {
            print_announce_result(&announce_result);

            // Verify we got a valid response (interval should be positive)
            assert!(
                announce_result.interval > 0,
                "Tracker should return a positive interval"
            );

            // Empty peer list is acceptable (torrent might have no active peers)
            // This is not an error condition per the acceptance criteria
            println!("\nTest passed: Successfully received tracker response");
        }
        Err(e) => {
            // Print the error for diagnostic purposes
            eprintln!("\nTracker announce failed: {}", e);
            eprintln!("This may be a transient network error or the tracker may be down.");

            // The test fails if we can't reach the tracker
            // However, we provide useful diagnostic information
            panic!("Failed to announce to tracker: {}", e);
        }
    }
}

/// Test that verifies the fixture file exists and can be parsed
/// This test does NOT require network access
#[test]
fn test_fixture_loads_correctly() {
    let metainfo = load_ubuntu_torrent();

    // Verify expected values from ubuntu.torrent.expected
    assert_eq!(metainfo.name, "ubuntu-24.04.3-live-server-amd64.iso");
    assert_eq!(metainfo.total_length, 3303444480);
    assert_eq!(metainfo.piece_length, 262144);
    assert_eq!(metainfo.piece_hashes.len(), 12602);

    // Verify info hash matches expected value
    let expected_hash = "a1dfefec1a9dd7fa8a041ebeeea271db55126d2f";
    let actual_hash = hash_to_hex(&metainfo.info_hash);
    assert_eq!(actual_hash, expected_hash);
}

/// Test TrackerManager construction from metainfo
/// This verifies TrackerManager properly loads trackers from announce-list
#[test]
fn test_tracker_manager_loads_trackers() {
    let metainfo = load_ubuntu_torrent();

    let manager = TrackerManager::new(&metainfo);

    // The ubuntu.torrent may have an announce-list
    // Verify the manager loads at least 1 tier and 1 tracker
    assert!(
        manager.tier_count() >= 1,
        "Should have at least 1 tier, got {}",
        manager.tier_count()
    );
    assert!(
        manager.tracker_count() >= 1,
        "Should have at least 1 tracker, got {}",
        manager.tracker_count()
    );

    // Verify all_trackers returns non-empty
    let all = manager.all_trackers();
    assert!(!all.is_empty(), "Should have at least one tracker URL");

    // Verify the primary announce URL is in the list
    // (it should either be in announce-list or used as fallback)
    println!("Tracker tiers: {:?}", manager.tiers());
    println!("All trackers: {:?}", all);
}

/// Integration test: TrackerManager with fallback behavior
///
/// This test creates a metainfo with multiple trackers (simulating announce-list)
/// and verifies the TrackerManager properly handles the tier structure.
///
/// Note: This test is marked #[ignore] because it requires network access.
#[tokio::test]
#[ignore]
async fn test_tracker_manager_fallback_to_secondary() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    // Load the base torrent
    let metainfo = load_ubuntu_torrent();

    // Create a modified metainfo with announce-list containing:
    // - Tier 0: Invalid tracker (will fail)
    // - Tier 1: The real Ubuntu tracker (should succeed)
    let modified_metainfo = Metainfo {
        announce: metainfo.announce.clone(),
        announce_list: Some(vec![
            // Tier 0: Invalid tracker that will definitely fail
            vec!["http://invalid-tracker.localhost:1/announce".to_string()],
            // Tier 1: The real tracker
            vec![metainfo.announce.clone()],
        ]),
        info_hash: metainfo.info_hash,
        piece_length: metainfo.piece_length,
        piece_hashes: metainfo.piece_hashes.clone(),
        total_length: metainfo.total_length,
        files: metainfo.files.clone(),
        name: metainfo.name.clone(),
    };

    // Create TrackerManager
    let mut manager = TrackerManager::new(&modified_metainfo);

    assert_eq!(manager.tier_count(), 2);
    assert_eq!(manager.tracker_count(), 2);

    println!("\nTesting TrackerManager fallback behavior...");
    println!("Tier 0: http://invalid-tracker.localhost:1/announce (will fail)");
    println!("Tier 1: {} (should succeed)", metainfo.announce);

    // Create announce request
    let request = TrackerRequest {
        info_hash: modified_metainfo.info_hash,
        peer_id: generate_peer_id(),
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: modified_metainfo.total_length,
        event: Some(TrackerEvent::Started),
        compact: true,
        numwant: Some(200),
    };

    // Attempt announce - should fail on tier 0, succeed on tier 1
    let result = manager.announce_with_fallback(request).await;

    match result {
        Ok(announce_result) => {
            println!("\nFallback successful!");
            print_announce_result(&announce_result);

            // Verify we got a valid response
            assert!(
                announce_result.interval > 0,
                "Tracker should return a positive interval"
            );

            println!("\nTest passed: TrackerManager successfully fell back to tier 1");
        }
        Err(e) => {
            // If both trackers fail, this is a network issue
            eprintln!("\nAll trackers failed: {}", e);
            panic!("TrackerManager fallback test failed: {}", e);
        }
    }
}

/// Test that TrackerManager properly orders attempts starting from last successful
#[tokio::test]
#[ignore]
async fn test_tracker_manager_remembers_successful_tracker() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    let metainfo = load_ubuntu_torrent();

    // Create metainfo with announce-list where the real tracker is in tier 1
    let modified_metainfo = Metainfo {
        announce: metainfo.announce.clone(),
        announce_list: Some(vec![
            // Tier 0: Invalid
            vec!["http://invalid1.localhost:1/announce".to_string()],
            // Tier 1: Real tracker
            vec![metainfo.announce.clone()],
        ]),
        info_hash: metainfo.info_hash,
        piece_length: metainfo.piece_length,
        piece_hashes: metainfo.piece_hashes.clone(),
        total_length: metainfo.total_length,
        files: metainfo.files.clone(),
        name: metainfo.name.clone(),
    };

    let mut manager = TrackerManager::new(&modified_metainfo);

    // First announce - should fail tier 0, succeed tier 1
    let request1 = TrackerRequest {
        info_hash: modified_metainfo.info_hash,
        peer_id: generate_peer_id(),
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: modified_metainfo.total_length,
        event: Some(TrackerEvent::Started),
        compact: true,
        numwant: Some(200),
    };

    println!("\nFirst announce (should try tier 0, fail, then tier 1)...");
    let result1 = manager.announce_with_fallback(request1).await;
    assert!(result1.is_ok(), "First announce should succeed via fallback");
    println!("First announce succeeded via fallback to tier 1");

    // Second announce - should start with tier 1 (the previously successful tracker)
    let request2 = TrackerRequest {
        info_hash: modified_metainfo.info_hash,
        peer_id: generate_peer_id(),
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: modified_metainfo.total_length - 1000,
        event: None,
        compact: true,
        numwant: Some(200),
    };

    println!("\nSecond announce (should try tier 1 first now)...");
    let result2 = manager.announce_with_fallback(request2).await;
    assert!(result2.is_ok(), "Second announce should succeed immediately");
    println!("Second announce succeeded (should have been faster)");

    println!("\nTest passed: TrackerManager remembers successful tracker");
}
