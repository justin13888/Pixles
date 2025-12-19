use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    Image(ImageMediaType),
    Video(VideoMediaType),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageMediaType {
    Jpeg,
    Jxl,
    Heic,
    Png,
    Tiff,
    Avif,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageOutputSettings {
    Jpeg(JpegSettings),
    Jxl(JxlSettings),
    Heic(HeicSettings),
    Png(PngSettings),
    Tiff(TiffSettings),
    Avif(AvifSettings),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JpegSettings {
    pub quality: u8,            // 1-100
    pub chroma_subsampling: u8, // 444, 422, 420
    pub progressive: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JxlSettings {
    pub distance: f32, // 0.0 (lossless) to 15.0
    pub effort: u8,    // 1-9
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeicSettings {
    pub crf: u8,   // 0-51
    pub speed: u8, // 0-9
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PngSettings {
    pub compression_level: u8, // 0-9
    pub bit_depth: u8,         // 8 or 16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TiffSettings {
    pub compression: TiffCompression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TiffCompression {
    None,
    Lzw,
    Deflate,
    Jpeg(u8), // Quality
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvifSettings {
    pub quality: u8,       // 0-63
    pub alpha_quality: u8, // 0-63
    pub speed: u8,         // 0-10
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoMediaType {
    Mp4,
    Webm,
    Mov,
    Avi,
    Mkv,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoOutputSettings {
    Mp4(Mp4Settings),
    Webm(WebmSettings),
    Mov(MovSettings),
    Avi(AviSettings),
    Mkv(MkvSettings),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mp4Settings {
    pub codec: H264Codec,
    pub rate_control: RateControl,
    pub preset: VideoPreset,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebmSettings {
    pub codec: Vp9Codec,
    pub crf: u8,          // 0-63 for VP9
    pub deadline: String, // "good", "best", "realtime"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovSettings {
    pub profile: ProResProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AviSettings {
    pub bitrate_kbps: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MkvSettings {
    // MKV is a "Swiss Army Knife", usually mirrors HEVC/AV1 settings
    pub crf: u8,
    pub preset: VideoPreset,
}

// Support Enums for Video
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RateControl {
    Crf(u8),
    Bitrate { target: u32, max: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoPreset {
    Ultrafast,
    Superfast,
    Veryfast,
    Faster,
    Fast,
    Medium,
    Slow,
    Slower,
    Veryslow,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProResProfile {
    Proxy,
    Lt,
    Standard,
    Hq,
    Xq,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum H264Codec {
    H264,
    H265,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Vp9Codec {
    Vp8,
    Vp9,
    Av1,
}
