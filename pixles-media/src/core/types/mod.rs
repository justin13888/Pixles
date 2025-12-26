use serde::{Deserialize, Serialize};

// Re-export image types for backward compatibility
pub use crate::image::types::*;
// Re-export video types for backward compatibility
pub use crate::video::types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    Image(ImageFormat),
    Video(VideoFormat),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Rational<T: num_traits::Unsigned + Copy> {
    pub numerator: T,
    pub denominator: T,
}
