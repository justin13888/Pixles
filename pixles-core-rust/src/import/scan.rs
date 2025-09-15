use serde::{Deserialize, Serialize};

use crate::{
    import::{SpecialDirectoryStatus, SpecialFileStatus},
    metadata::AssetType,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanResult {
    File {
        detected_asset_type: Option<AssetType>,
        is_special: Option<SpecialFileStatus>,
    },
    Directory {
        detected_asset_type: Option<AssetType>,
        is_special: Option<SpecialDirectoryStatus>,
    },
}
