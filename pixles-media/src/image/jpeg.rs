use std::path::Path;

use crate::image::{Image, ImageMetadata, RGBAImage};

#[derive(Debug, Clone)]
pub struct JpegImage {}

impl Image for JpegImage {
    fn get_rgba(&self) -> RGBAImage {
        unimplemented!()
    }

    fn get_metadata(&self) -> ImageMetadata {
        unimplemented!()
    }

    async fn from_path(path: &Path) -> Result<Box<Self>, String> {
        unimplemented!()
    }
}
