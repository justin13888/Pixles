use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::image::{metadata::ImageMetadata, rgba::RGBAImage};

pub mod formats;
pub mod lqip;
pub mod metadata;
pub mod rgba;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageFile {
    pub path: PathBuf,
    pub metadata: ImageMetadata,
}

pub trait Image: std::fmt::Debug + Send + Sync {
    fn get_rgba(&self) -> RGBAImage;
    fn get_metadata(&self) -> ImageMetadata;
    fn from_path(
        path: &Path,
    ) -> impl std::future::Future<Output = Result<Box<Self>, String>> + Send
    where
        Self: Sized;
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
