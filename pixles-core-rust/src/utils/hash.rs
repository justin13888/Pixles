use std::{fs, io, path::Path};

/// Get hash of a file
// TODO: Consider making this async (probably doesn't make sense at all for throughput)
pub fn get_file_hash(path: &Path) -> io::Result<u64> {
    // TODO: switch to streaming version for large files
    let bytes = fs::read(path)?;
    Ok(xxhash_rust::xxh3::xxh3_64(&bytes))
}
