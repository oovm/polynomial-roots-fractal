use std::{fs::File, path::Path};

fn main() -> ImageResult<()> {
    let width = 3000;
    let height = 3000;
    let mut image_buffer: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    for pixel in image_buffer.pixels_mut() {
        let r: u16 = rng.gen_range(0..=u16::MAX);
        let g: u16 = rng.gen_range(0..=u16::MAX);
        let b: u16 = rng.gen_range(0..=u16::MAX);
        let a: u16 = rng.gen_range(0..=u16::MAX);
        *pixel = Rgba { 0: [r, g, b, a] };
    }
    let file_path = Path::new("output.png");
    let file = File::create(file_path)?;
    let encoder = PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::NoFilter);
    encoder.encode(&DynamicImage::ImageRgba16(image_buffer).to_bytes(), width, height, ColorType::Rgba16)?;
    Ok(())
}
