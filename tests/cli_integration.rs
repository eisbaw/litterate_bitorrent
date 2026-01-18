// CLI integration tests for the BitTorrent client.
//
// These tests verify the CLI behavior without requiring network access.
// They test argument parsing, validation, and error handling.
//
// Run with: cargo test --test cli_integration

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

use tempfile::tempdir;

/// Path to the compiled binary
fn binary_path() -> PathBuf {
    // First try release build, then debug
    let release = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/release/literate-bittorrent");
    if release.exists() {
        return release;
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/debug/literate-bittorrent")
}

/// Run the binary with given arguments
fn run_binary(args: &[&str]) -> Output {
    Command::new(binary_path())
        .args(args)
        .output()
        .expect("Failed to execute binary")
}

/// Path to the test fixture torrent file
fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/ubuntu.torrent")
}

#[test]
fn test_help_flag() {
    let output = run_binary(&["--help"]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show help text
    assert!(stdout.contains("Usage:") || stdout.contains("literate-bittorrent"),
        "Help should show usage: {}", stdout);
    assert!(stdout.contains("torrent") || stdout.contains("TORRENT"),
        "Help should mention torrent argument: {}", stdout);
}

#[test]
fn test_version_flag() {
    let output = run_binary(&["--version"]);

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show version
    assert!(stdout.contains("literate-bittorrent") || stdout.contains("0.1"),
        "Version should be displayed: {}", stdout);
}

#[test]
fn test_missing_torrent_argument() {
    let output = run_binary(&[]);

    // Should fail with error about missing argument
    assert!(!output.status.success(), "Should fail without torrent argument");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("argument") || stderr.contains("TORRENT"),
        "Should mention missing required argument: {}", stderr);
}

#[test]
fn test_invalid_torrent_file() {
    let output = run_binary(&["nonexistent.torrent"]);

    // Should fail with error about file not found
    assert!(!output.status.success(), "Should fail with nonexistent file");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("exist") || stderr.contains("Error"),
        "Should mention file not found: {}", stderr);
}

#[test]
fn test_invalid_torrent_content() {
    let temp = tempdir().expect("create temp dir");
    let invalid_torrent = temp.path().join("invalid.torrent");

    // Write invalid content
    fs::write(&invalid_torrent, b"not a valid torrent file").expect("write file");

    let output = run_binary(&[invalid_torrent.to_str().unwrap()]);

    // Should fail with parse error
    assert!(!output.status.success(), "Should fail with invalid torrent content");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error") || stderr.contains("parse") || stderr.contains("invalid"),
        "Should mention parse error: {}", stderr);
}

#[test]
fn test_verbose_flag() {
    // This test just verifies the -v flag is accepted
    // We use --help with -v to avoid actually starting a download
    let output = run_binary(&["--help", "-v"]);

    // Help should still work with verbose flag
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:") || stdout.contains("literate-bittorrent"),
        "Help should work with verbose flag: {}", stdout);
}

#[test]
fn test_custom_output_directory() {
    let temp = tempdir().expect("create temp dir");
    let output_dir = temp.path().join("output");

    // Create an invalid torrent so it fails quickly after argument parsing
    let invalid_torrent = temp.path().join("test.torrent");
    fs::write(&invalid_torrent, b"d4:infod4:name4:test6:lengthi100e12:piece lengthi100e6:pieces20:xxxxxxxxxxxxxxxxxxxx8:announce10:http://a/ee")
        .expect("write file");

    let output = run_binary(&[
        invalid_torrent.to_str().unwrap(),
        "-o", output_dir.to_str().unwrap(),
    ]);

    // The command will fail (no real tracker), but we're just testing argument parsing
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should not complain about the -o flag being invalid
    assert!(!stderr.contains("unrecognized") && !stderr.contains("invalid option"),
        "Should accept -o flag: stdout={} stderr={}", stdout, stderr);
}

#[test]
fn test_custom_port() {
    let temp = tempdir().expect("create temp dir");

    // Create an invalid torrent so it fails quickly after argument parsing
    let invalid_torrent = temp.path().join("test.torrent");
    fs::write(&invalid_torrent, b"d4:infod4:name4:test6:lengthi100e12:piece lengthi100e6:pieces20:xxxxxxxxxxxxxxxxxxxx8:announce10:http://a/ee")
        .expect("write file");

    let output = run_binary(&[
        invalid_torrent.to_str().unwrap(),
        "-p", "6889",
    ]);

    // The command will fail (no real tracker), but we're just testing argument parsing
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should not complain about the -p flag being invalid
    assert!(!stderr.contains("unrecognized") && !stderr.contains("invalid option"),
        "Should accept -p flag: stdout={} stderr={}", stdout, stderr);
}

/// Test that valid torrent file is recognized (but will fail on tracker)
/// This test requires the ubuntu.torrent fixture and network access.
/// It is marked #[ignore] because it may hang waiting for tracker response.
/// Run explicitly with: cargo test --test cli_integration -- --ignored
#[test]
#[ignore]
fn test_valid_torrent_file_recognized() {
    let fixture = fixture_path();
    if !fixture.exists() {
        eprintln!("Skipping test: ubuntu.torrent fixture not found");
        return;
    }

    let temp = tempdir().expect("create temp dir");

    let output = run_binary(&[
        fixture.to_str().unwrap(),
        "-o", temp.path().to_str().unwrap(),
    ]);

    // The download will fail (tracker/network issues), but it should get past
    // argument parsing and torrent loading
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show torrent info (meaning it parsed successfully)
    let combined = format!("{}{}", stdout, stderr);

    // Either it shows torrent info OR it fails on tracker/network
    // It should NOT fail on "not found" or "parse error" for the torrent
    assert!(
        combined.contains("ubuntu") ||
        combined.contains("tracker") ||
        combined.contains("Tracker") ||
        combined.contains("peer") ||
        combined.contains("Piece"),
        "Should recognize valid torrent file: stdout={} stderr={}", stdout, stderr
    );
}
