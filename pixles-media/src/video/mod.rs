use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoFile {
    pub path: PathBuf,
    pub metadata: VideoMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoMetadata {
    // TODO: Add fields
}
