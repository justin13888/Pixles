use std::path::Path;

use jpeg_encoder::{ColorType, Encoder as JpegEncoderStruct};
use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;

use crate::image::{EncodeError, Image, ImageEncode, ImageMetadata, RGBAImage};
use crate::metadata::ColorSpace as PxColorSpace;

#[derive(Debug, Clone)]
pub struct JpegImage {
    width: u16,
    height: u16,
    data: Vec<u8>,
    file_size_bytes: u64,
}

impl Image for JpegImage {
    fn get_rgba(&self) -> RGBAImage {
        // We assume data is RGBA because we decode it that way
        RGBAImage::from_bytes(self.data.clone(), self.width as usize, self.height as usize)
            .expect("Failed to create RGBAImage from internal data")
    }

    fn get_metadata(&self) -> ImageMetadata {
        ImageMetadata {
            format: "JPEG".to_string(),
            file_size_bytes: self.file_size_bytes,
            width: self.width as u32,
            height: self.height as u32,
            bit_depth: 8,
            color_space: PxColorSpace::Srgb,
            ..Default::default()
        }
    }

    async fn from_path(path: &Path) -> Result<Box<Self>, String> {
        let file_content = tokio::fs::read(path).await.map_err(|e| e.to_string())?;
        let file_size_bytes = file_content.len() as u64;

        let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);
        let mut decoder = JpegDecoder::new_with_options(&file_content, options);

        let data = decoder
            .decode()
            .map_err(|e| format!("JPEG decode error: {:?}", e))?;

        let info = decoder.info().ok_or("Failed to get image info")?;

        Ok(Box::new(Self {
            width: info.width,
            height: info.height,
            data,
            file_size_bytes,
        }))
    }
}

impl ImageEncode for JpegImage {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buf = Vec::new();
        let encoder = JpegEncoderStruct::new(&mut buf, 80);

        encoder
            .encode(&self.data, self.width, self.height, ColorType::Rgba)
            .map_err(|e| EncodeError::FailedToEncode(e.to_string()))?;

        Ok(buf)
    }
}
