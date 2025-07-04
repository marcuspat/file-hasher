use hex::encode;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::io::{self, BufReader, Read};

const BUFFER_SIZE: usize = 1024 * 1024; // 1MB buffer for streaming

pub fn compute_sha256<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut hasher = Sha256::new();
    stream_to_hasher(reader, &mut hasher)?;
    Ok(encode(hasher.finalize()))
}

pub fn compute_md5<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut buf_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut context = md5::Context::new();

    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", context.compute()))
}

pub fn compute_sha1<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut hasher = Sha1::new();
    stream_to_hasher(reader, &mut hasher)?;
    Ok(encode(hasher.finalize()))
}

fn stream_to_hasher<R: Read, D: Digest>(reader: &mut R, hasher: &mut D) -> io::Result<()> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut buf_reader = BufReader::with_capacity(BUFFER_SIZE, reader);

    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(())
}
