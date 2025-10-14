use image::{ImageBuffer, Rgb, RgbImage};
use std::path::Path;

#[derive(Default)]
pub struct Image {
    pub aspect_ratio: f64,
    pub width: u32,
    pub height: u32,
    pub buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(aspect_ratio: f64, width: u32) -> Self {
        let height = ((width as f64 / aspect_ratio) as u32).max(1);
        Self {
            aspect_ratio,
            width,
            height,
            buffer: RgbImage::new(width, height),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        self.buffer.save(path).unwrap()
    }
}
