
use std::fmt;
use std::error::Error;
use std::fs::{File};
use std::io::{Read};

use memmap::MmapOptions;
use crc32fast::Hasher;

#[derive(Debug)]
pub enum Digest {
    CRC32
}

type Result<T> = std::result::Result<T, DigestError>;

#[derive(Debug)]
pub struct DigestError {
    message: String
}

impl DigestError {
    fn new(msg: &str) -> DigestError {
        DigestError{ message: msg.to_string() }
    }
}

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DigestError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub fn calculate_digest(mut file: File, _digest: Digest) -> Result<u32> {

    let filesize = match file.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => { return Err(DigestError::new("error getting file size")); }
    };

    let mut buffer = Vec::with_capacity(filesize);
    buffer.resize(filesize, 0);
    let _bytes_read = match file.read_exact(&mut buffer) {
        Ok(n) => n,
        Err(_) => { return Err(DigestError::new("error reading file contents")); }
    };

    let mut hasher = Hasher::new();
    hasher.update(&buffer);

    Ok(hasher.finalize())
}

pub fn calculate_digest_mmap(file: File, _digest: Digest) -> Result<u32> {

    let buffer = unsafe {
        match MmapOptions::new().map(&file) {
            Ok(b) => b,
            Err(_) => { return Err(DigestError::new("error reading file contents")) }
        }
    };

    let mut hasher = Hasher::new();
    hasher.update(&buffer);

    Ok(hasher.finalize())
}
