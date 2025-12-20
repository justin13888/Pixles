#[derive(Debug, Clone)]
pub struct RGBAImage {
    inner: Vec<u8>, // TODO: Support non-rgba and 8+ bit depth in the future as generic
    pub width: usize,
    pub height: usize,
}

impl AsRef<RGBAImage> for RGBAImage {
    fn as_ref(&self) -> &RGBAImage {
        self
    }
}

impl RGBAImage {
    pub fn from_bytes(inner: Vec<u8>, width: usize, height: usize) -> Result<RGBAImage, String> {
        if inner.len() != width * height * 4 {
            return Err("Invalid image data size".to_string());
        }
        Ok(RGBAImage {
            inner,
            width,
            height,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.inner
    }

    /// Resizes the image to the given width and height
    /// It is typically used to downsize an image for thumbnail generation
    pub fn resize(&self, width: usize, height: usize) -> RGBAImage {
        // For upscaling, give option for Bicubic and Bilinear
        // For downscaling, give option for Lanczos3 and Mitchell-Netravali
        // implementation could use image or zune-image
        unimplemented!()
    }
}
