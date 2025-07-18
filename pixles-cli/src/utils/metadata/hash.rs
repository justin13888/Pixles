use std::path::Path;
use std::{fs, io};

/// Get hash of a file
pub fn get_file_hash(path: &Path) -> io::Result<u64> {
    // TODO: switch to streaming version for large files
    let bytes = fs::read(path)?;
    Ok(xxhash_rust::xxh3::xxh3_64(&bytes))
}
