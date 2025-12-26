use std::path::Path;

use jpeg_encoder::{ColorType, Encoder as JpegEncoderStruct};
use zune_core::colorspace::ColorSpace as ZuneColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;

use crate::image::{
    Image, ImageDecode, ImageEncode, ImageError, ImageMetadata,
    buffer::{ComponentType, ImageBuffer, PixelFormat},
};
use crate::metadata::ColorSpace;

#[derive(Debug, Clone)]
pub struct JpegImage {
    width: u16,
    height: u16,
    data: Vec<u8>,
    format: PixelFormat,
    color_space: ColorSpace,
    file_size_bytes: u64,
}

impl Image for JpegImage {
    fn get_buffer(&self) -> ImageBuffer {
        ImageBuffer::new(
            self.data.clone(),
            self.width as usize,
            self.height as usize,
            self.format,
            ComponentType::U8,
            self.color_space,
        )
        .expect("Failed to create ImageBuffer from internal data")
    }

    fn get_metadata(&self) -> ImageMetadata {
        ImageMetadata {
            format: "JPEG".to_string(),
            file_size_bytes: self.file_size_bytes,
            width: self.width as u32,
            height: self.height as u32,
            bit_depth: 8,
            color_space: self.color_space,
            ..Default::default() // TODO: jpeg-encoder exposes much more metadata. Finish this properly.
        }
    }

    fn from_raw_parts(buffer: ImageBuffer, _metadata: ImageMetadata) -> Result<Self, ImageError> {
        // JPEGs represent 8-bit data.
        let buffer = buffer.into_rgba8()?;

        let width = buffer.width as u16;
        let height = buffer.height as u16;
        let format = buffer.format;
        let data = buffer.data; // transfer ownership

        Ok(Self {
            width,
            height,
            data,
            format,
            color_space: buffer.color_space,
            file_size_bytes: 0, // Generated image, no file size yet
        })
    }
}

impl ImageDecode for JpegImage {
    fn decode_from_bytes(data: &[u8]) -> Result<Self, ImageError> {
        // Decode headers
        let mut decoder = JpegDecoder::new(std::io::Cursor::new(data));
        decoder
            .decode_headers()
            .map_err(|e| ImageError::Decode(format!("{:?}", e)))?;
        let info = decoder
            .info()
            .ok_or(ImageError::Decode("Failed to get image info".to_string()))?;

        // Determine output based on component count
        let (out_colorspace, pixel_format) = match info.components {
            1 => (ZuneColorSpace::Luma, PixelFormat::Gray),
            4 => (ZuneColorSpace::CMYK, PixelFormat::Cmyk),
            _ => (ZuneColorSpace::RGB, PixelFormat::Rgb),
        };

        // Initialize decoder with specific output options
        let options = DecoderOptions::default().jpeg_set_out_colorspace(out_colorspace);
        let mut decoder =
            JpegDecoder::new_with_options(zune_core::bytestream::ZCursor::new(data), options);

        let decoded_data = decoder
            .decode()
            .map_err(|e| ImageError::Decode(format!("{:?}", e)))?;

        let info = decoder
            .info()
            .ok_or(ImageError::Decode("Failed to get image info".to_string()))?;

        Ok(Self {
            width: info.width,
            height: info.height,
            data: decoded_data,
            format: pixel_format,
            color_space: ColorSpace::Srgb, // Assuming SRGB for now
            file_size_bytes: data.len() as u64,
        })
    }
}

impl ImageEncode for JpegImage {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), ImageError> {
        let encoder = JpegEncoderStruct::new(writer, 80); // Quality 80

        let color_type = match self.format {
            PixelFormat::Gray => ColorType::Luma,
            PixelFormat::Rgb => ColorType::Rgb,
            PixelFormat::Rgba => ColorType::Rgba,
            PixelFormat::Cmyk => ColorType::Cmyk,
        };

        encoder
            .encode(&self.data, self.width, self.height, color_type)
            .map_err(|e| ImageError::Encode(e.to_string()))?;
        Ok(())
    }

    async fn save(&self, path: &Path) -> Result<(), ImageError> {
        let data = self.encode_to_bytes()?;
        tokio::fs::write(path, data).await.map_err(ImageError::Io)
    }
}
