use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_directory_input_fails_with_nonzero_exit() {
    let dir = tempdir().unwrap();

    Command::cargo_bin("file-hasher")
        .unwrap()
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_missing_file_among_many_still_hashes_the_rest() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("real.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"hash me")
        .unwrap();

    // The remaining file must still be hashed, but the run must exit non-zero
    Command::cargo_bin("file-hasher")
        .unwrap()
        .arg("definitely-missing.txt")
        .arg(file_path.to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains("real.txt"))
        .stdout(predicate::str::contains("SHA256"))
        .stderr(predicate::str::contains("not found"));
}
