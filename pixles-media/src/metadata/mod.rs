use serde::{Deserialize, Serialize};

pub mod geo;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ColorSpace {
    #[default]
    Srgb,
    AdobeRgb,
    DisplayP3,
    ProPhoto,
    Linear,
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceMetadata {
    pub make: String,         // e.g., "Sony"
    pub model: String,        // e.g., "A7IV"
    pub lens: Option<String>, // e.g., "FE 35mm F1.4 GM"
    pub serial_number: Option<String>,
    pub software_version: Option<String>,
}
