use file_hasher::{compute_all_with_progress, compute_md5, compute_sha1, compute_sha256};
use std::io::{self, Cursor, Read};

/// A reader that fails if read() is called again after EOF was returned once,
/// proving the data was streamed exactly one time.
struct SinglePassReader {
    data: Vec<u8>,
    pos: usize,
    eof_returned: bool,
}

impl SinglePassReader {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: 0,
            eof_returned: false,
        }
    }
}

impl Read for SinglePassReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.data.len() {
            if self.eof_returned {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "read called after EOF: input streamed more than once",
                ))
            } else {
                self.eof_returned = true;
                Ok(0)
            }
        } else {
            let n = std::cmp::min(buf.len(), self.data.len() - self.pos);
            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Ok(n)
        }
    }
}

#[test]
fn test_all_mode_reads_input_exactly_once() {
    let data = b"single pass hashing\n".repeat(200_000); // ~3.8MB, spans multiple 1MB chunks
    let mut reader = SinglePassReader::new(data.clone());

    let hashes = compute_all_with_progress(&mut reader, |_| {})
        .expect("all-mode hashing failed on a single-pass reader");

    // Digests must match the per-algorithm results over the same data
    let mut expected_sha256_reader = Cursor::new(data.clone());
    let expected_sha256 = compute_sha256(&mut expected_sha256_reader).unwrap();
    let mut expected_md5_reader = Cursor::new(data.clone());
    let expected_md5 = compute_md5(&mut expected_md5_reader).unwrap();
    let mut expected_sha1_reader = Cursor::new(data.clone());
    let expected_sha1 = compute_sha1(&mut expected_sha1_reader).unwrap();

    assert_eq!(hashes.sha256, expected_sha256);
    assert_eq!(hashes.md5, expected_md5);
    assert_eq!(hashes.sha1, expected_sha1);
}

#[test]
fn test_progress_callback_reports_every_byte() {
    let data = vec![0xCDu8; 2_500_000]; // 2.5MB, forces a partial final chunk
    let mut reader = Cursor::new(data.clone());

    let mut reported: u64 = 0;
    let hashes = compute_all_with_progress(&mut reader, |n| reported += n).unwrap();

    assert_eq!(reported, 2_500_000, "callback must sum to the input size");
    assert!(!hashes.sha256.is_empty());
}

#[test]
fn test_single_algorithm_progress_callback_reports_every_byte() {
    let data = vec![0xABu8; 1_500_000];
    let mut reader = Cursor::new(data);

    let mut reported: u64 = 0;
    let hash = file_hasher::compute_sha256_with_progress(&mut reader, |n| reported += n).unwrap();

    assert_eq!(reported, 1_500_000);
    assert_eq!(hash.len(), 64);
}
