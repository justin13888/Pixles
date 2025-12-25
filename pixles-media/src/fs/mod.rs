use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::fs;

use crate::{
    core::types::{ImageMediaType, MediaType, VideoMediaType},
    image::{Image, ImageDecode, ImageFile, formats::jpeg::JpegImage, rgba::RGBAImage},
    video::VideoFile,
};

pub mod ext;

async fn is_path_file(path: &Path) -> Result<bool, std::io::Error> {
    let metadata = fs::metadata(path).await?;
    Ok(metadata.is_file())
}

/// Reads a media file from the given path and returns a MediaFile enum.
pub async fn read(file_path: &Path) -> Result<MediaFile, ReadMediaError> {
    // Verify it is a file
    if !is_path_file(file_path).await? {
        return Err(ReadMediaError::NotAFile);
    }

    let media_type: MediaType = ext::detect_media_type(file_path)
        .await?
        .ok_or(ReadMediaError::UnknownFormat)?;

    // Parse based on media type
    let mf = match media_type {
        MediaType::Image(t) => MediaFile::Image(read_image(file_path, t).await?),
        MediaType::Video(t) => MediaFile::Video(read_video(file_path, t).await?),
    };

    Ok(mf)
}

async fn read_image(file_path: &Path, t: ImageMediaType) -> Result<ImageFile, ReadMediaError> {
    unimplemented!()
}

async fn read_video(file_path: &Path, t: VideoMediaType) -> Result<VideoFile, ReadMediaError> {
    unimplemented!()
}

#[derive(Error, Debug)]
pub enum ReadMediaError {
    #[error("Path is not a file")]
    NotAFile,
    #[error("Unknown media format")]
    UnknownFormat,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MediaFile {
    Image(ImageFile),
    Video(VideoFile),
}

/// Detects the image type from a path
///
/// Returns [ReadImageError] if the path is not an image file.
pub async fn detect_image_type(path: &Path) -> Result<ImageMediaType, ReadImageError> {
    // Verify it is a file
    if !is_path_file(path).await? {
        return Err(ReadImageError::NotAFile);
    }

    let media_type: MediaType = ext::detect_media_type(path)
        .await?
        .ok_or(ReadImageError::UnknownFormat)?;

    match media_type {
        MediaType::Image(t) => Ok(t),
        _ => Err(ReadImageError::NotAnImage(media_type)),
    }
}

#[derive(Error, Debug)]
pub enum ReadImageError {
    #[error("Path is not a file")]
    NotAFile,
    #[error("Unknown media format")]
    UnknownFormat,
    #[error("Not an image")]
    NotAnImage(MediaType),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ImageParseError {
    #[error("Read image error: {0}")]
    ReadImageError(#[from] ReadImageError),
    #[error("Image error: {0}")]
    ImageError(#[from] crate::image::ImageError),
    #[error("Image data error: {0}")]
    DataError(String),
}

impl From<String> for ImageParseError {
    fn from(s: String) -> Self {
        ImageParseError::DataError(s)
    }
}

/// Load an image into memory
pub async fn load_image(path: &Path) -> Result<Box<dyn Image>, ImageParseError> {
    // Identify the image type
    let image_type = detect_image_type(path).await?;

    // Parse the image bytes
    let image: Box<dyn Image> = match image_type {
        ImageMediaType::Jpeg => Box::new(JpegImage::from_path(path).await?),
        ImageMediaType::Jxl => unimplemented!(),
        ImageMediaType::Heic => unimplemented!(),
        ImageMediaType::Png => unimplemented!(),
        ImageMediaType::Tiff => unimplemented!(),
        ImageMediaType::Avif => unimplemented!(),
        ImageMediaType::WebP => unimplemented!(),
        ImageMediaType::Gif => unimplemented!(),
        ImageMediaType::Bmp => unimplemented!(),
        ImageMediaType::Raw(t) => unimplemented!(),
    };

    Ok(image)
}

/// Load an image into memory as an RGBA image
pub async fn load_image_as_rgba(path: &Path) -> Result<RGBAImage, ImageParseError> {
    let image = load_image(path).await?;
    let rgba = image.get_rgba();

    Ok(rgba)
}
