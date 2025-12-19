use std::{io::Read, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::fs;

use crate::{
    core::types::{ImageMediaType, MediaType, VideoMediaType},
    image::ImageFile,
    video::VideoFile,
};

pub mod ext;

/// Reads a media file from the given path and returns a MediaFile enum.
pub async fn read(file_path: &Path) -> Result<MediaFile, ReadMediaError> {
    // 1. Verify it is a file
    let metadata = fs::metadata(file_path).await?;
    if !metadata.is_file() {
        return Err(ReadMediaError::NotAFile);
    }

    let media_type: MediaType = ext::detect_media_type(file_path)
        .await?
        .ok_or(ReadMediaError::UnsupportedFormat)?;

    // Parse based on media type
    let mf = match media_type {
        MediaType::Image(t) => MediaFile::Image(read_image(file_path, t).await?),
        MediaType::Video(t) => MediaFile::Video(read_video(file_path, t).await?),
    };

    Ok(mf)
}

pub async fn read_image(file_path: &Path, t: ImageMediaType) -> Result<ImageFile, ReadMediaError> {
    unimplemented!()
}

pub async fn read_video(file_path: &Path, t: VideoMediaType) -> Result<VideoFile, ReadMediaError> {
    unimplemented!()
}

#[derive(Error, Debug)]
pub enum ReadMediaError {
    #[error("Path is not a file")]
    NotAFile,
    #[error("Unsupported media format")]
    UnsupportedFormat,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MediaFile {
    Image(ImageFile),
    Video(VideoFile),
}
