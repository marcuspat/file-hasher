use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_cli_single_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello, CLI!").unwrap();

    let mut cmd = Command::cargo_bin("file-hasher").unwrap();
    cmd.arg(file_path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("SHA256"));
}

#[test]
fn test_cli_multiple_files() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");

    File::create(&file1).unwrap().write_all(b"File 1").unwrap();
    File::create(&file2).unwrap().write_all(b"File 2").unwrap();

    let mut cmd = Command::cargo_bin("file-hasher").unwrap();
    cmd.args(&[file1.to_str().unwrap(), file2.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));
}

#[test]
fn test_cli_algorithm_selection() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    File::create(&file_path)
        .unwrap()
        .write_all(b"Test")
        .unwrap();

    // Test MD5
    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&["--algorithm", "md5", file_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("MD5"));

    // Test SHA1
    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&["--algorithm", "sha1", file_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("SHA1"));

    // Test All
    Command::cargo_bin("file-hasher")
        .unwrap()
        .args(&["--algorithm", "all", file_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("SHA256"))
        .stdout(predicate::str::contains("MD5"))
        .stdout(predicate::str::contains("SHA1"));
}

#[test]
fn test_cli_invalid_file() {
    Command::cargo_bin("file-hasher")
        .unwrap()
        .arg("nonexistent.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}
