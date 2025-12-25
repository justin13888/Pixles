use std::path::Path;

use jpeg_encoder::{ColorType, Encoder as JpegEncoderStruct};
use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;

use crate::image::{Image, ImageDecode, ImageEncode, ImageError, ImageMetadata, RGBAImage};
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
}

impl ImageDecode for JpegImage {
    fn decode<R: std::io::BufRead>(mut reader: R) -> Result<Self, ImageError> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).map_err(ImageError::Io)?;

        // JpegDecoder expects a slice
        let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);
        let mut decoder = JpegDecoder::new_with_options(std::io::Cursor::new(&buffer), options);

        let data = decoder
            .decode()
            .map_err(|e| ImageError::Decode(format!("{:?}", e)))?;

        let info = decoder
            .info()
            .ok_or(ImageError::Decode("Failed to get image info".to_string()))?;

        Ok(Self {
            width: info.width,
            height: info.height,
            data,
            file_size_bytes: buffer.len() as u64,
        })
    }
}

impl ImageEncode for JpegImage {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ImageError> {
        let mut encoder = JpegEncoderStruct::new(writer, 80); // Quality 80
        encoder
            .encode(&self.data, self.width, self.height, ColorType::Rgba)
            .map_err(|e| ImageError::Encode(e.to_string()))?;
        Ok(())
    }

    async fn save(&self, path: &Path) -> Result<(), ImageError> {
        let data = self.encode_to_bytes()?;
        tokio::fs::write(path, data).await.map_err(ImageError::Io)
    }
}
