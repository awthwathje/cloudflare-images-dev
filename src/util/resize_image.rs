use image::{imageops::FilterType, ImageError, ImageFormat};
use std::path::PathBuf;

pub async fn resize_image(
    original_path: PathBuf,
    variant_path: PathBuf,
    width: u32,
    height: u32,
) -> Result<(), ImageError> {
    let img = image::open(original_path)?;
    let resized = img.resize(width, height, FilterType::Nearest);
    resized.save_with_format(variant_path, ImageFormat::Jpeg)
}
