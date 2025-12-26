use std::io::Write;
use std::path::Path;

use crate::core::types::RawImageMediaType;
use crate::image::{
    Image, ImageDecode, ImageEncode, ImageError, ImageMetadata, buffer::ImageBuffer,
};

#[derive(Debug, Clone)]
pub struct RawImage {
    pub subtype: RawImageMediaType,
}

impl RawImage {
    pub async fn from_path(
        _path: impl AsRef<Path>,
        subtype: RawImageMediaType,
    ) -> Result<Self, ImageError> {
        // Placeholder for potentially reading metadata or validation
        Ok(Self { subtype })
    }
}

impl Image for RawImage {
    fn get_buffer(&self) -> ImageBuffer {
        unimplemented!()
    }

    fn get_metadata(&self) -> ImageMetadata {
        unimplemented!()
    }

    fn from_raw_parts(_buffer: ImageBuffer, _metadata: ImageMetadata) -> Result<Self, ImageError> {
        unimplemented!()
    }
}

impl ImageDecode for RawImage {
    fn decode_from_bytes(_bytes: &[u8]) -> Result<Self, ImageError> {
        unimplemented!()
    }
}

impl ImageEncode for RawImage {
    fn encode<W: Write>(&self, _writer: &mut W) -> Result<(), ImageError> {
        unimplemented!()
    }

    async fn save(&self, _path: &Path) -> Result<(), ImageError> {
        unimplemented!()
    }
}
