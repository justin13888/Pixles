use std::path::Path;

use crate::image::{EncodeError, Image, ImageEncode, ImageMetadata, RGBAImage};

#[derive(Debug, Clone)]
pub struct WebpImage {}

impl Image for WebpImage {
    fn get_rgba(&self) -> RGBAImage {
        unimplemented!()
    }

    fn get_metadata(&self) -> ImageMetadata {
        unimplemented!()
    }

    async fn from_path(_path: &Path) -> Result<Box<Self>, String> {
        unimplemented!()
    }
}

impl ImageEncode for WebpImage {
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        unimplemented!()
    }
}
