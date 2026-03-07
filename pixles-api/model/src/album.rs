use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlbumAccess {
    Owner,
    Write,
    Read,
}

impl AlbumAccess {
    /// Returns whether user has write access to album.
    pub fn is_write(&self) -> bool {
        matches!(self, AlbumAccess::Owner | AlbumAccess::Write)
    }
}
