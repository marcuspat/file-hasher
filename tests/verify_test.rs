use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_verify_correct_hash() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"Hello, World!")
        .unwrap();

    // Known SHA256 hash for "Hello, World!"
    let correct_hash = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";

    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&["--verify", correct_hash, file_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("MATCH"));
}

#[test]
fn test_verify_incorrect_hash() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"Hello, World!")
        .unwrap();

    let incorrect_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&["--verify", incorrect_hash, file_path.to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("MISMATCH"));
}

#[test]
fn test_verify_with_algorithm() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"Hello, World!")
        .unwrap();

    // Known MD5 hash for "Hello, World!"
    let md5_hash = "65a8e27d8879283831b664bd8b7f0ad4";

    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&[
            "--algorithm",
            "md5",
            "--verify",
            md5_hash,
            file_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("MATCH"));
}

#[test]
fn test_verify_multiple_files() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");

    File::create(&file1).unwrap().write_all(b"File 1").unwrap();
    File::create(&file2).unwrap().write_all(b"File 2").unwrap();

    // This should fail as we can't verify one hash against multiple files
    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&[
            "--verify",
            "somehash",
            file1.to_str().unwrap(),
            file2.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("multiple files"));
}
