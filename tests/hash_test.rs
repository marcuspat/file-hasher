use file_hasher::{compute_md5, compute_sha1, compute_sha256};
use std::io::Cursor;

#[test]
fn test_sha256_computation() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let hash = compute_sha256(&mut reader).expect("Failed to compute SHA256");
    assert_eq!(
        hash,
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    );
}

#[test]
fn test_md5_computation() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let hash = compute_md5(&mut reader).expect("Failed to compute MD5");
    assert_eq!(hash, "65a8e27d8879283831b664bd8b7f0ad4");
}

#[test]
fn test_sha1_computation() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let hash = compute_sha1(&mut reader).expect("Failed to compute SHA1");
    assert_eq!(hash, "0a0a9f2a6772942557ab5355d76af442f8f65e01");
}

#[test]
fn test_empty_input() {
    let data = b"";
    let mut reader = Cursor::new(data);
    let hash = compute_sha256(&mut reader).expect("Failed to compute SHA256");
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}
