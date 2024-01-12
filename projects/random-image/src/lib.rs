use clap::Parser;
use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    ColorType, DynamicImage, ImageBuffer, ImageResult, Rgba,
};
use rand::Rng;
use std::{fs::File, io::Write, path::Path};

#[derive(Debug, Parser)]
pub struct RandomImage {
    #[arg(default_value = "1024")]
    width: u32,
    #[arg(default_value = "1024")]
    height: u32,
}

impl RandomImage {
    pub fn run(&self) -> ImageResult<()> {
        let mut image_buffer: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(self.width, self.height);
        let mut rng = rand::thread_rng();
        for pixel in image_buffer.pixels_mut() {
            let r: u16 = rng.gen_range(0..=u16::MAX);
            let g: u16 = rng.gen_range(0..=u16::MAX);
            let b: u16 = rng.gen_range(0..=u16::MAX);
            let a: u16 = rng.gen_range(0..=u16::MAX);
            *pixel = Rgba { 0: [r, g, b, a] };
        }
        let file_path = Path::new("image_raw.png");
        let file = File::create(file_path)?;
        let encoder = PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::NoFilter);
        encoder.encode(&DynamicImage::ImageRgba16(image_buffer).as_bytes(), self.width, self.height, ColorType::Rgba16)?;
        Ok(())
    }
}
// #[test]
// fn main() -> std::io::Result<()> {
//     let file_path = "image_raw.png";
//     let file_size: u64 = 1 * 1024 * 1024 * 1024; // 文件大小为 83GB
//     let chunk_size: u64 = 1024 * 1024; // 每次写入的块大小为 1MB
//
//     let mut file = File::create(file_path)?;
//
//     let mut written = 0;
//     let mut buffer = vec![0u8; chunk_size as usize];
//
//     while written < file_size {
//         let remaining = file_size - written;
//         let write_size = std::cmp::min(remaining, chunk_size);
//
//         file.write_all(&buffer[..write_size as usize])?;
//         written += write_size;
//     }
//
//     Ok(())
// }
