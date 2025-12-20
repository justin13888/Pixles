use thiserror::Error;

use crate::image::{resize_to_max_dimension, rgba::RGBAImage};

/// LQIP (thumbhash) struct
pub struct LQIP(Vec<u8>);

impl LQIP {
    pub fn from_bytes(bytes: Vec<u8>) -> LQIP {
        LQIP(bytes)
    }

    /// Generate a LQIP (thumbhash) from RGBA image
    ///
    /// You do not need to resize as it will be done to input internally.
    /// Returns a byte sequence
    pub async fn from_rgba_image<T>(rgba: T) -> LQIP
    where
        T: AsRef<RGBAImage>,
    {
        let original_rgba = rgba.as_ref();
        let mut rgba_view = std::borrow::Cow::Borrowed(original_rgba);

        // Downsize if any dimension is larger than MAX_SIZE
        const MAX_SIZE: usize = 100;
        if original_rgba.width > MAX_SIZE || original_rgba.height > MAX_SIZE {
            let (new_width, new_height) =
                resize_to_max_dimension(original_rgba.width, original_rgba.height, MAX_SIZE);
            let new_rgba = original_rgba.resize(new_width, new_height);
            rgba_view = std::borrow::Cow::Owned(new_rgba);
        }

        let bytes =
            thumbhash::rgba_to_thumb_hash(rgba_view.width, rgba_view.height, rgba_view.as_bytes());
        LQIP::from_bytes(bytes)
    }

    /// Extracts the approximate aspect ratio of the original image
    pub fn approx_aspect_ratio(&self) -> Result<f32, LQIPParseError> {
        thumbhash::thumb_hash_to_approximate_aspect_ratio(&self.0)
            .map_err(|_| LQIPParseError::InvalidHash)
    }

    /// Extracts the average color (r,g,b,a) from a ThumbHash
    pub fn average_rgba(&self) -> Result<[f32; 4], LQIPParseError> {
        let (r, g, b, a) = thumbhash::thumb_hash_to_average_rgba(&self.0)
            .map_err(|_| LQIPParseError::InvalidHash)?;
        Ok([r, g, b, a])
    }

    /// Decodes a ThumbHash to an RGBA image.
    pub fn thumb_hash_to_rgba(&self) -> Result<RGBAImage, LQIPParseError> {
        let (width, height, rgba) =
            thumbhash::thumb_hash_to_rgba(&self.0).map_err(|_| LQIPParseError::InvalidHash)?;
        let rgba = RGBAImage::from_bytes(rgba, width, height)
            .map_err(|_| LQIPParseError::UnhandledState)?;
        Ok(rgba)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Error, Debug)]
pub enum LQIPParseError {
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Unhandled state")]
    UnhandledState,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_solid_rgba(width: usize, height: usize, r: u8, g: u8, b: u8, a: u8) -> RGBAImage {
        let mut data = Vec::with_capacity(width * height * 4);
        for _ in 0..(width * height) {
            data.extend_from_slice(&[r, g, b, a]);
        }
        RGBAImage::from_bytes(data, width, height).unwrap()
    }

    #[tokio::test]
    async fn test_approx_aspect_ratio_square() {
        let rgba = create_solid_rgba(50, 50, 255, 0, 0, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let ar = lqip.approx_aspect_ratio().unwrap();
        // Square image should have aspect ratio close to 1.0
        assert!(
            (ar - 1.0).abs() < 0.2,
            "Aspect ratio {} was not close to 1.0",
            ar
        );
    }

    #[tokio::test]
    async fn test_approx_aspect_ratio_landscape() {
        let rgba = create_solid_rgba(100, 50, 0, 255, 0, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let ar = lqip.approx_aspect_ratio().unwrap();
        // 2:1 landscape image
        assert!(
            (ar - 2.0).abs() < 0.4,
            "Aspect ratio {} was not close to 2.0",
            ar
        );
    }

    #[tokio::test]
    async fn test_approx_aspect_ratio_portrait() {
        let rgba = create_solid_rgba(50, 100, 0, 0, 255, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let ar = lqip.approx_aspect_ratio().unwrap();
        // 1:2 portrait image
        assert!(
            (ar - 0.5).abs() < 0.2,
            "Aspect ratio {} was not close to 0.5",
            ar
        );
    }

    #[tokio::test]
    async fn test_average_rgba_red() {
        let rgba = create_solid_rgba(80, 80, 255, 0, 0, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let avg = lqip.average_rgba().unwrap();
        // Red component should be high, others low
        assert!(avg[0] > 0.8, "Red component {} too low", avg[0]);
        assert!(avg[1] < 0.2, "Green component {} too high", avg[1]);
        assert!(avg[2] < 0.2, "Blue component {} too high", avg[2]);
        assert!(avg[3] > 0.9, "Alpha component {} too low", avg[3]);
    }

    #[tokio::test]
    async fn test_average_rgba_semi_transparent() {
        let rgba = create_solid_rgba(80, 80, 0, 255, 0, 128);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let avg = lqip.average_rgba().unwrap();
        // Alpha should be around 0.5
        assert!(
            (avg[3] - 0.5).abs() < 0.1,
            "Alpha component {} not close to 0.5",
            avg[3]
        );
    }

    #[tokio::test]
    async fn test_round_trip_reconstruction() {
        let rgba = create_solid_rgba(64, 64, 100, 150, 200, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let decoded = lqip
            .thumb_hash_to_rgba()
            .expect("Should decode back to RGBA");

        // Detailed equality is hard with ThumbHash lossy compression,
        // but we can check if average color remains similar.
        let lqip2 = LQIP::from_rgba_image(&decoded).await;
        let avg1 = lqip.average_rgba().unwrap();
        let avg2 = lqip2.average_rgba().unwrap();

        for i in 0..4 {
            assert!(
                (avg1[i] - avg2[i]).abs() < 0.1,
                "Average color mismatch at index {}",
                i
            );
        }
    }

    #[test]
    fn test_invalid_hash_handling() {
        let lqip = LQIP::from_bytes(vec![0, 1, 2]); // Too short
        assert!(lqip.approx_aspect_ratio().is_err());
        assert!(lqip.average_rgba().is_err());
        assert!(lqip.thumb_hash_to_rgba().is_err());
    }

    #[tokio::test]
    async fn test_minimal_image() {
        let rgba = create_solid_rgba(1, 1, 255, 255, 255, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        assert!(lqip.approx_aspect_ratio().is_ok());
    }

    #[tokio::test]
    async fn test_dimensions_preserved() {
        // Test that landscape aspect ratio is preserved in reconstructed image dimensions
        let rgba = create_solid_rgba(60, 30, 255, 255, 255, 255);
        let lqip = LQIP::from_rgba_image(&rgba).await;
        let decoded = lqip.thumb_hash_to_rgba().unwrap();

        // ThumbHash might not return EXACTLY 60x30, but it should be a landscape
        assert!(
            decoded.width > decoded.height,
            "Reconstructed image should be landscape ({}x{})",
            decoded.width,
            decoded.height
        );
        let ar = decoded.width as f32 / decoded.height as f32;
        assert!(
            (ar - 2.0).abs() < 0.5,
            "Reconstructed aspect ratio {} should be near 2.0",
            ar
        );
    }
}
