use file_hasher::{hash_file, HashAlgorithm};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[test]
fn test_large_file_streaming() {
    // Create a 50MB temporary file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let data = vec![0u8; 1024 * 1024]; // 1MB chunk
    for _ in 0..50 {
        temp_file.write_all(&data).expect("Failed to write data");
    }
    temp_file.flush().expect("Failed to flush");

    // Hash the file
    let path = temp_file.path();
    let hash = hash_file(path, HashAlgorithm::SHA256).expect("Failed to hash file");

    // Verify it completes without running out of memory
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 64); // SHA256 produces 64 hex characters
}

#[test]
fn test_multiple_algorithm_streaming() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(b"Test data for multiple algorithms")
        .expect("Failed to write");
    temp_file.flush().expect("Failed to flush");

    let path = temp_file.path();

    let sha256 = hash_file(path, HashAlgorithm::SHA256).expect("Failed to hash SHA256");
    let md5 = hash_file(path, HashAlgorithm::MD5).expect("Failed to hash MD5");
    let sha1 = hash_file(path, HashAlgorithm::SHA1).expect("Failed to hash SHA1");

    assert_eq!(sha256.len(), 64);
    assert_eq!(md5.len(), 32);
    assert_eq!(sha1.len(), 40);
}

#[test]
fn test_nonexistent_file() {
    let result = hash_file(Path::new("nonexistent.txt"), HashAlgorithm::SHA256);
    assert!(result.is_err());
}
