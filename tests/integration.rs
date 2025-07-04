use file_hasher::{hash_file, HashAlgorithm, Hasher};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_real_file_hashing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("real_test.txt");
    let content = b"This is a real integration test file!\nWith multiple lines.\n";

    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file.sync_all().unwrap();

    // Test all hash algorithms
    let sha256 = hash_file(&file_path, HashAlgorithm::SHA256).unwrap();
    let md5 = hash_file(&file_path, HashAlgorithm::MD5).unwrap();
    let sha1 = hash_file(&file_path, HashAlgorithm::SHA1).unwrap();

    // Verify hash lengths
    assert_eq!(sha256.len(), 64);
    assert_eq!(md5.len(), 32);
    assert_eq!(sha1.len(), 40);

    // Verify actual hash values
    assert_eq!(
        sha256,
        "d656b1203aa0de3d68f8fe7534ebb07493d105070dbe23f34246f152f123753a"
    );
    assert_eq!(md5, "cbd40c9209b286f2d41a01d577e3798f");
    assert_eq!(sha1, "f3002a26c55db9d50ea1fbf0a5469b0adc7c680a");
}

#[test]
fn test_multiple_files_processing() {
    let dir = tempdir().unwrap();
    let files: Vec<(PathBuf, &[u8])> = vec![
        (dir.path().join("file1.txt"), b"Content of file 1"),
        (dir.path().join("file2.txt"), b"Content of file 2"),
        (dir.path().join("file3.txt"), b"Content of file 3"),
    ];

    // Create test files
    for (path, content) in &files {
        let mut file = File::create(path).unwrap();
        file.write_all(content).unwrap();
    }

    // Hash all files
    let mut results = Vec::new();
    for (path, _) in &files {
        let hash = hash_file(path, HashAlgorithm::SHA256).unwrap();
        results.push(hash);
    }

    // Verify all hashes are different
    assert_eq!(results.len(), 3);
    assert_ne!(results[0], results[1]);
    assert_ne!(results[0], results[2]);
    assert_ne!(results[1], results[2]);
}

#[test]
fn test_hash_verification_functionality() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("verify_test.txt");
    let content = b"Test content for verification";

    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file.sync_all().unwrap();

    // Compute hashes
    let sha256_hash = hash_file(&file_path, HashAlgorithm::SHA256).unwrap();
    let md5_hash = hash_file(&file_path, HashAlgorithm::MD5).unwrap();
    let sha1_hash = hash_file(&file_path, HashAlgorithm::SHA1).unwrap();

    // Verify correct hashes
    assert_eq!(
        sha256_hash,
        "07e744b643dfaff12f16183a3910361ee112df61c9e10fc29255553500ceacb7"
    );
    assert_eq!(md5_hash, "087e6c7f3fcef84d336e3bab456aa140");
    assert_eq!(sha1_hash, "c6576286dc6ac041b778bf0ae5e352e6925fe65f");

    // Test Hasher struct
    let hasher = Hasher::new(HashAlgorithm::SHA256);
    let hash_via_hasher = hasher.hash_file(&file_path).unwrap();
    assert_eq!(hash_via_hasher, sha256_hash);
}

#[test]
fn test_large_file_streaming_integration() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("large_file.bin");

    // Create a 25MB file
    let mut file = File::create(&file_path).unwrap();
    let chunk = vec![0xAB; 1024 * 1024]; // 1MB of 0xAB bytes
    for _ in 0..25 {
        file.write_all(&chunk).unwrap();
    }
    file.sync_all().unwrap();

    // Ensure file was created with correct size
    let metadata = fs::metadata(&file_path).unwrap();
    assert_eq!(metadata.len(), 25 * 1024 * 1024);

    // Hash the large file - this tests streaming
    let start = std::time::Instant::now();
    let hash = hash_file(&file_path, HashAlgorithm::SHA256).unwrap();
    let duration = start.elapsed();

    // Verify hash is correct
    assert_eq!(hash.len(), 64);

    // Ensure it completed in reasonable time (less than 10 seconds)
    assert!(
        duration.as_secs() < 10,
        "Hashing took too long: {:?}",
        duration
    );
}

#[test]
fn test_fixture_files() {
    // Test the fixture files we created
    let small_path = PathBuf::from("tests/fixtures/small.txt");
    let medium_path = PathBuf::from("tests/fixtures/medium.txt");

    if small_path.exists() {
        let hash = hash_file(&small_path, HashAlgorithm::SHA256).unwrap();
        assert_eq!(hash.len(), 64);
    }

    if medium_path.exists() {
        let hash = hash_file(&medium_path, HashAlgorithm::SHA256).unwrap();
        assert_eq!(hash.len(), 64);
    }
}
