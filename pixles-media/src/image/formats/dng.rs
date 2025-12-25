use std::io::{BufRead, Write};
use std::path::Path;

use crate::image::{Image, ImageDecode, ImageEncode, ImageError, ImageMetadata, RGBAImage};

#[derive(Debug, Clone)]
pub struct DngImage {}

impl Image for DngImage {
    fn get_rgba(&self) -> RGBAImage {
        unimplemented!()
    }

    fn get_metadata(&self) -> ImageMetadata {
        unimplemented!()
    }
}

impl ImageDecode for DngImage {
    fn decode<R: BufRead>(_reader: R) -> Result<Self, ImageError> {
        unimplemented!()
    }
}

impl ImageEncode for DngImage {
    fn encode<W: Write>(&self, _writer: &mut W) -> Result<(), ImageError> {
        unimplemented!()
    }

    async fn save(&self, _path: &Path) -> Result<(), ImageError> {
        unimplemented!()
    }
}
