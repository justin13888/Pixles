use std::{
    io::{BufRead, Cursor, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::image::{buffer::ImageBuffer, metadata::ImageMetadata};

pub mod buffer;
pub mod formats;
pub mod lqip;
pub mod metadata;

#[derive(Debug)]
pub struct ImageFile {
    pub source_path: PathBuf,
    pub image: Box<dyn Image>,
}

pub trait Image: std::fmt::Debug + Send + Sync {
    /// Returns the raw pixel buffer of the image.
    ///
    /// This method allows access to the underlying pixel buffer. The format and characteristics
    /// of the buffer can be inspected via the `ImageBuffer` fields.
    fn get_buffer(&self) -> ImageBuffer;

    /// Retrieves metadata associated with the image.
    ///
    /// This includes information such as dimensions, color space, and file format specifics.
    fn get_metadata(&self) -> ImageMetadata;

    /// Creates a new image from a raw pixel buffer and metadata.
    ///
    /// This allows initializing an image directly from its components (pixel data and metadata),
    /// useful for format conversion or generating images programmatically.
    fn from_raw_parts(buffer: ImageBuffer, metadata: ImageMetadata) -> Result<Self, ImageError>
    where
        Self: Sized;
}

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Decoding error: {0}")]
    Decode(String),
    #[error("Encoding error: {0}")]
    Encode(String),
    #[error("Image buffer error: {0}")]
    ImageBuffer(#[from] crate::image::buffer::ImageBufferError),
}

pub trait ImageDecode: Sized + Image + 'static {
    /// Decodes an image directly from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice containing the encoded image data.
    ///
    /// # Returns
    ///
    /// * `Result<Self, ImageError>` - The decoded image instance or an error.
    fn decode_from_bytes(data: &[u8]) -> Result<Self, ImageError>;
}

pub trait ImageReader: ImageDecode {
    /// Decodes an image from a buffered reader.
    ///
    /// This is the core decoding method. It accepts any type implementing `BufRead` (e.g., `BufReader<File>`, `Cursor<Vec<u8>>`),
    /// allowing for flexible input sources.
    ///
    /// # Arguments
    ///
    /// * `reader` - A buffered reader providing the image data.
    ///
    /// # Returns
    ///
    /// * `Result<Self, ImageError>` - The decoded image instance or an error.
    fn decode<R: BufRead>(mut reader: R) -> Result<Self, ImageError> {
        // We need to decode the whole blob so we first read it all into memory.
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map_err(ImageError::Io)?;
        Self::decode_from_bytes(&buf)
    }

    /// Asynchronously decodes an image from a file path.
    ///
    /// This method reads the file at the specified path asynchronously and then decodes it.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    ///
    /// * `impl Future<Output = Result<Self, ImageError>>` - A future resolving to the decoded image or an error.
    fn from_path(path: impl AsRef<Path>) -> impl Future<Output = Result<Self, ImageError>> + Send {
        let path = path.as_ref().to_owned();
        async move {
            // Use spawn_blocking to protect the executor from disk latency
            tokio::task::spawn_blocking(move || {
                let file = std::fs::File::open(&path).map_err(ImageError::Io)?;
                let meta = file.metadata().map_err(ImageError::Io)?;

                // Heuristic: Mmap large files, read small ones
                if meta.len() > 16 * 1024 {
                    // File is larger than 16KB
                    // TODO: Validate this heuristic is optimal
                    let mmap = unsafe { memmap2::Mmap::map(&file).map_err(ImageError::Io)? };
                    Self::decode_from_bytes(&mmap)
                } else {
                    let mut buf = Vec::with_capacity(meta.len() as usize);
                    std::io::Read::read_to_end(&mut &file, &mut buf).map_err(ImageError::Io)?;
                    Self::decode_from_bytes(&buf)
                }
            })
            .await
            .map_err(|e| ImageError::Decode(format!("JoinError: {e:?}")))?
        }
    }
}

impl<T: ImageDecode> ImageReader for T {}

pub trait ImageEncode: Image + Sync {
    /// Encodes the image to a writer.
    ///
    /// This is the core encoding method. It writes the encoded image data to the provided writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A writable target (e.g., `File`, `Vec<u8>`, `TcpStream`).
    ///
    /// # Returns
    ///
    /// * `Result<(), ImageError>` - `Ok(())` on success, or an error on failure.
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), ImageError>;

    /// Encodes the image to a vector of bytes.
    ///
    /// A convenience wrapper around `encode` that returns the encoded data as a `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, ImageError>` - The encoded image bytes or an error.
    fn encode_to_bytes(&self) -> Result<Vec<u8>, ImageError> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    /// Asynchronously saves the encoded image to a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The destination path for the image file.
    ///
    /// # Returns
    ///
    /// * `impl Future<Output = Result<(), ImageError>>` - A future resolving to `Ok(())` or an error.
    fn save(&self, path: &Path) -> impl Future<Output = Result<(), ImageError>> + Send;
}

/// Returns dimensions that maintain aspect ratio while ensuring the largest dimension is at most target_max
pub fn resize_to_max_dimension(w: usize, h: usize, target_max: usize) -> (usize, usize) {
    // Determine the scale factor based on the larger dimension
    let larger_dimension = if h > w { h } else { w };
    if larger_dimension <= target_max {
        return (w, h);
    }
    let scale = target_max as f64 / (larger_dimension as f64);

    // Apply scale and round to maintain aspect ratio integrity
    let w_resized = (w as f64 * scale).round() as usize;
    let h_resized = (h as f64 * scale).round() as usize;

    (w_resized, h_resized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_to_max_dimension() {
        // Landscape
        let (w, h) = resize_to_max_dimension(200, 100, 100);
        assert_eq!(w, 100);
        assert_eq!(h, 50);

        // Portrait
        let (w, h) = resize_to_max_dimension(100, 200, 100);
        assert_eq!(w, 50);
        assert_eq!(h, 100);

        // Square
        let (w, h) = resize_to_max_dimension(150, 150, 100);
        assert_eq!(w, 100);
        assert_eq!(h, 100);

        // Already smaller
        let (w, h) = resize_to_max_dimension(50, 50, 100);
        assert_eq!(w, 50);
        assert_eq!(h, 50);
    }
}
