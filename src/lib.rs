pub mod hash;

use std::io;
use std::path::Path;

pub use hash::{compute_md5, compute_sha1, compute_sha256};

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
    let mut file = std::fs::File::open(path)?;

    match algorithm {
        HashAlgorithm::SHA256 => compute_sha256(&mut file),
        HashAlgorithm::MD5 => compute_md5(&mut file),
        HashAlgorithm::SHA1 => compute_sha1(&mut file),
    }
}
