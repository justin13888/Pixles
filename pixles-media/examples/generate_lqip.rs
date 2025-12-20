use std::path::PathBuf;

use pixles_media::image::{Image, formats::jpeg::JpegImage, lqip::LQIP};

#[tokio::main]
pub async fn main() {
    let image_path = PathBuf::from("./data/test.jpg");
    println!("Image path: {}", image_path.display());
    let image: Box<dyn Image> = JpegImage::from_path(&image_path)
        .await
        .expect("Failed to load image");
    let rgba = image.get_rgba();
    let lqip = LQIP::from_rgba_image(&rgba).await;
    let lqip_as_hex: String = lqip
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" ");
    println!("LQIP: {}", lqip_as_hex);
    println!("Average color: {:?}", lqip.average_rgba().unwrap());
    println!(
        "Approximate aspect ratio: {:?}",
        lqip.approx_aspect_ratio().unwrap()
    );
}
