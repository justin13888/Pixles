use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::types::{
    ImageMediaType, ImageOutputSettings, VideoMediaType, VideoOutputSettings,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TranscodeTask {
    Image(ImageTranscodeTask),
    Video(VideoTranscodeTask),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageTranscodeTask {
    pub input: PathBuf,
    pub input_type: ImageMediaType,
    pub output: PathBuf,
    pub output_settings: ImageOutputSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoTranscodeTask {
    pub input: PathBuf,
    pub input_type: VideoMediaType,
    pub output: PathBuf,
    pub output_settings: VideoOutputSettings,
}
