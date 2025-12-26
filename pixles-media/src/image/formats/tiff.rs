use std::io::Write;
use std::path::Path;

use crate::image::{
    Image, ImageDecode, ImageEncode, ImageError, ImageMetadata, buffer::ImageBuffer,
};

#[derive(Debug, Clone)]
pub struct TiffImage {}

impl Image for TiffImage {
    fn get_format(&self) -> crate::core::types::ImageFormat {
        crate::core::types::ImageFormat::Tiff
    }

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

impl ImageDecode for TiffImage {
    fn decode_from_bytes(_bytes: &[u8]) -> Result<Self, ImageError> {
        unimplemented!()
    }
}

impl ImageEncode for TiffImage {
    fn encode<W: Write>(&self, _writer: &mut W) -> Result<(), ImageError> {
        unimplemented!()
    }

    async fn save(&self, _path: &Path) -> Result<(), ImageError> {
        unimplemented!()
    }
}
