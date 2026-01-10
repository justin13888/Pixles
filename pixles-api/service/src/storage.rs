use pixles_core::models::asset::Asset;

use std::path::PathBuf;

#[derive(Clone)]
pub struct StorageConfig {
    pub upload_dir: PathBuf,
}

#[derive(Clone)]
pub struct StorageService {
    config: StorageConfig,
}

impl StorageService {
    pub fn new(config: StorageConfig) -> Self {
        Self { config }
    }

    /// Returns the normalized path for the asset
    pub fn get_upload_path(&self, asset: Asset) -> PathBuf {
        let Asset {
            id: asset_id,
            album_id,
            owner_id,
            ext,
        } = asset;
        let asset_id_encoded = data_encoding::BASE32_NOPAD.encode(asset_id.as_bytes());

        let mut dir = self.config.upload_dir.join(owner_id);
        if let Some(album_id) = album_id {
            dir = dir.join(album_id);
        }

        dir = dir.join(format!("{asset_id_encoded}.{ext}"));

        dir
    }

    pub fn get_upload_dir(&self, upload_id: &str) -> PathBuf {
        self.config.upload_dir.join(upload_id)
    }
}
