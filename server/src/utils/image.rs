use image::DynamicImage;
use std::{error::Error, fs};
use webp::{Encoder, WebPMemory};

fn quality_range_protector(quality: f32) -> Result<f32, String> {
    if (0.0..=100.0).contains(&quality) {
        Ok(quality)
    } else {
        Err("Quality must be between 0 and 100".to_string())
    }
}

pub fn convert_to_webp(
    input_image_path: &str,
    output_image_path: &str,
    quality: f32,
) -> Result<(), Box<dyn Error>> {
    let img: DynamicImage = image::open(input_image_path)?;
    let encoder: Encoder<'_> =
        Encoder::from_image(&img).map_err(|e| format!("Failed to create a webp encoder: {}", e))?;
    let webp: WebPMemory = encoder.encode(quality_range_protector(quality)?);
    fs::write(output_image_path, webp.to_vec())?;
    Ok(())
}
