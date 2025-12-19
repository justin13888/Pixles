use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MediaType {
    Image(ImageMediaType),
    Video(VideoMediaType),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageMediaType {
    Jpeg,
    Jxl,
    Heic,
    Png,
    Tiff,
    Avif,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoMediaType {
    Mp4,
    Webm,
    Mov,
    Avi,
    Mkv,
}
