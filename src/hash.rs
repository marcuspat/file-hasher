use hex::encode;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::io::{self, Read};

const BUFFER_SIZE: usize = 1024 * 1024; // 1MB buffer for streaming

/// All three digests, computed in a single pass over the data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllHashes {
    pub sha256: String,
    pub md5: String,
    pub sha1: String,
}

pub fn compute_sha256<R: Read>(reader: &mut R) -> io::Result<String> {
    compute_sha256_with_progress(reader, |_| {})
}

pub fn compute_sha256_with_progress<R: Read, F: FnMut(u64)>(
    reader: &mut R,
    on_chunk: F,
) -> io::Result<String> {
    let mut hasher = Sha256::new();
    stream_to_hasher(reader, &mut hasher, on_chunk)?;
    Ok(encode(hasher.finalize()))
}

pub fn compute_md5<R: Read>(reader: &mut R) -> io::Result<String> {
    compute_md5_with_progress(reader, |_| {})
}

pub fn compute_md5_with_progress<R: Read, F: FnMut(u64)>(
    reader: &mut R,
    mut on_chunk: F,
) -> io::Result<String> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut context = md5::Context::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
        on_chunk(bytes_read as u64);
    }

    Ok(format!("{:x}", context.compute()))
}

pub fn compute_sha1<R: Read>(reader: &mut R) -> io::Result<String> {
    compute_sha1_with_progress(reader, |_| {})
}

pub fn compute_sha1_with_progress<R: Read, F: FnMut(u64)>(
    reader: &mut R,
    on_chunk: F,
) -> io::Result<String> {
    let mut hasher = Sha1::new();
    stream_to_hasher(reader, &mut hasher, on_chunk)?;
    Ok(encode(hasher.finalize()))
}

/// Feed every chunk to all three algorithms so the input is read exactly once.
pub fn compute_all_with_progress<R: Read, F: FnMut(u64)>(
    reader: &mut R,
    mut on_chunk: F,
) -> io::Result<AllHashes> {
    let mut sha256 = Sha256::new();
    let mut md5_context = md5::Context::new();
    let mut sha1 = Sha1::new();
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let chunk = &buffer[..bytes_read];
        sha256.update(chunk);
        md5_context.consume(chunk);
        sha1.update(chunk);
        on_chunk(bytes_read as u64);
    }

    Ok(AllHashes {
        sha256: encode(sha256.finalize()),
        md5: format!("{:x}", md5_context.compute()),
        sha1: encode(sha1.finalize()),
    })
}

fn stream_to_hasher<R: Read, D: Digest, F: FnMut(u64)>(
    reader: &mut R,
    hasher: &mut D,
    mut on_chunk: F,
) -> io::Result<()> {
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
        on_chunk(bytes_read as u64);
    }

    Ok(())
}
