use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageFile {
    pub path: PathBuf,
    pub metadata: ImageMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageMetadata {
    // TODO: Add fields
}
