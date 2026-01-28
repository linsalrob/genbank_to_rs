//! Integration tests for genbank_to_rs

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("genbank_to");
    path
}

fn get_test_data_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("data");
    path.push("input");
    path.push("NC_001417.gbk");
    path
}

#[test]
fn test_help_output() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Convert NCBI GenBank format files"));
    assert!(stdout.contains("--genbank"));
    assert!(stdout.contains("--aminoacids"));
}

#[test]
fn test_version_output() {
    let output = Command::new(get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_fasta_nucleotide_output() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.fna");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(get_test_data_path())
        .arg("-n")
        .arg(&output_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Output file was not created");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.starts_with(">NC_001417"));
    assert!(content.contains("gggtgggacccctttcggggtc")); // Start of sequence
}

#[test]
fn test_fasta_aminoacid_output() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.faa");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(get_test_data_path())
        .arg("-a")
        .arg(&output_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.starts_with(">YP_009640124.1"));
    assert!(content.contains("MRAFSTLDRENET")); // Start of protein sequence
}

#[test]
fn test_complex_headers() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test_complex.faa");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(get_test_data_path())
        .arg("-a")
        .arg(&output_path)
        .arg("--complex")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("[NC_001417]"));
    assert!(content.contains("[Escherichia phage MS2]"));
    assert!(content.contains("hypothetical protein"));
}

#[test]
fn test_ptt_output() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.ptt");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(get_test_data_path())
        .arg("-p")
        .arg(&output_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("YP_009640124.1"));
    assert!(content.contains("mat"));
}

#[test]
fn test_gzip_compression() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.fna");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(get_test_data_path())
        .arg("-n")
        .arg(&output_path)
        .arg("--zip")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists());

    // Check if file is gzip compressed
    let content = fs::read(&output_path).unwrap();
    assert_eq!(
        &content[0..2],
        &[0x1f, 0x8b],
        "File should be gzip compressed"
    );
}

#[test]
fn test_read_gzip_genbank_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.fna");
    
    // Path to gzipped GenBank file
    let mut gzip_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    gzip_path.push("tests");
    gzip_path.push("data");
    gzip_path.push("input");
    gzip_path.push("NC_001417.gbk.gz");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(&gzip_path)
        .arg("-n")
        .arg(&output_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Output file was not created");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.starts_with(">NC_001417"));
    assert!(content.contains("gggtgggacccctttcggggtc")); // Start of sequence
}

#[test]
fn test_read_gzip_genbank_file_aminoacids() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.faa");
    
    // Path to gzipped GenBank file
    let mut gzip_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    gzip_path.push("tests");
    gzip_path.push("data");
    gzip_path.push("input");
    gzip_path.push("NC_001417.gbk.gz");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(&gzip_path)
        .arg("-a")
        .arg(&output_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.starts_with(">YP_009640124.1"));
    assert!(content.contains("MRAFSTLDRENET")); // Start of protein sequence
}

#[test]
fn test_gzip_detection_with_uncompressed_file() {
    // This tests that regular uncompressed files still work
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test.fna");

    let output = Command::new(get_binary_path())
        .arg("-g")
        .arg(get_test_data_path())
        .arg("-n")
        .arg(&output_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", output);
    assert!(output_path.exists(), "Output file was not created");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.starts_with(">NC_001417"));
    assert!(content.contains("gggtgggacccctttcggggtc"));
}
