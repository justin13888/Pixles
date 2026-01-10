use chrono::{DateTime, Utc};
use entity::asset::AssetType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAsset {
    pub owner_id: String,
    pub album_id: Option<String>,
    pub width: i32,
    pub height: i32,
    pub asset_type: AssetType,
    pub original_filename: String,
    pub file_size: i64,
    pub file_hash: i64,
    pub content_type: String,
    pub date: Option<DateTime<Utc>>,
    pub uploaded: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAsset {
    pub album_id: Option<Option<String>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub asset_type: Option<AssetType>,
    pub original_filename: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<i64>,
    pub content_type: Option<String>,
    pub date: Option<Option<DateTime<Utc>>>,
    pub uploaded: Option<bool>,
    pub deleted_at: Option<Option<DateTime<Utc>>>,
}
