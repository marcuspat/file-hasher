pub mod hash;

use std::io;
use std::path::Path;

pub use hash::{
    compute_all_with_progress, compute_md5, compute_md5_with_progress, compute_sha1,
    compute_sha1_with_progress, compute_sha256, compute_sha256_with_progress, AllHashes,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashAlgorithm {
    SHA256,
    MD5,
    SHA1,
}

pub struct Hasher {
    algorithm: HashAlgorithm,
}

impl Hasher {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }

    pub fn hash_file(&self, path: &Path) -> io::Result<String> {
        hash_file(path, self.algorithm)
    }
}

pub fn hash_file(path: &Path, algorithm: HashAlgorithm) -> io::Result<String> {
    hash_file_with_progress(path, algorithm, |_| {})
}

/// Hash a file, reporting each streamed chunk's size to `on_chunk`.
pub fn hash_file_with_progress<F: FnMut(u64)>(
    path: &Path,
    algorithm: HashAlgorithm,
    on_chunk: F,
) -> io::Result<String> {
    let mut file = std::fs::File::open(path)?;

    match algorithm {
        HashAlgorithm::SHA256 => hash::compute_sha256_with_progress(&mut file, on_chunk),
        HashAlgorithm::MD5 => hash::compute_md5_with_progress(&mut file, on_chunk),
        HashAlgorithm::SHA1 => hash::compute_sha1_with_progress(&mut file, on_chunk),
    }
}

/// Hash a file once, producing all three digests.
pub fn hash_file_all(path: &Path) -> io::Result<AllHashes> {
    hash_file_all_with_progress(path, |_| {})
}

pub fn hash_file_all_with_progress<F: FnMut(u64)>(
    path: &Path,
    on_chunk: F,
) -> io::Result<AllHashes> {
    let mut file = std::fs::File::open(path)?;
    hash::compute_all_with_progress(&mut file, on_chunk)
}
