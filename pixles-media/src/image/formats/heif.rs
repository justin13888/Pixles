use std::path::Path;

use crate::image::{EncodeError, Image, ImageEncode, ImageMetadata, RGBAImage};

#[derive(Debug, Clone)]
pub struct HeifImage {}

impl Image for HeifImage {
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
