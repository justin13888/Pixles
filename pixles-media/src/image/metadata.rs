use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImageMetadata {
    // Basic File Information
    pub format: String, // e.g., "JPEG", "JPEG XL", "DNG"
    pub file_size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,           // 8, 10, 12, 14, 16
    pub color_space: ColorSpace, // sRGB, AdobeRGB, ProPhoto, etc.

    // Capture Device Information (EXIF)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceMetadata>,

    // Exposure & Optics (EXIF)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_settings: Option<CaptureSettings>,

    // Location (GPS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GpsLocation>,

    // Intellectual Property & Description (IPTC/XMP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentMetadata>,

    // Raw/Advanced Format Specifics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_info: Option<RawSpecifics>,

    // Catch-all for proprietary or format-specific tags
    pub extra_tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColorSpace {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaptureSettings {
    pub iso: u32,
    pub aperture: f32,         // f/1.8
    pub shutter_speed: String, // "1/250" (stored as string to preserve fraction)
    pub focal_length: f32,     // 35.0
    pub focal_length_35mm: Option<f32>,
    pub exposure_bias: f32,           // 0.0, -1.0, etc.
    pub white_balance: String,        // "Auto", "Manual", "5500K"
    pub capture_time: Option<String>, // ISO 8601 string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GpsLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
    pub copyright: Option<String>,
    pub rating: Option<u8>, // 0-5 stars
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawSpecifics {
    pub cfa_pattern: Option<String>, // e.g., "RGGB"
    pub black_level: Option<u32>,
    pub white_level: Option<u32>,
    pub is_compressed: bool,
}

impl Default for ColorSpace {
    fn default() -> Self {
        ColorSpace::Srgb
    }
}
