use engine_core::Color;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            data: vec![Color::TRANSPARENT; size],
        }
    }

    pub fn from_pixels(width: u32, height: u32, pixels: Vec<Color>) -> Self {
        Self {
            width,
            height,
            data: pixels,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = (y * self.width + x) as usize;
        Some(self.data[index])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.data[index] = color;
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = (u * self.width as f32).clamp(0.0, self.width as f32 - 1.0) as u32;
        let y = (v * self.height as f32).clamp(0.0, self.height as f32 - 1.0) as u32;
        self.get_pixel(x, y).unwrap_or(Color::TRANSPARENT)
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.data {
            *pixel = color;
        }
    }

    pub fn load_png<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let decoder = png::Decoder::new(std::fs::File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let mut pixels = Vec::with_capacity((info.width * info.height) as usize);

        match info.color_type {
            png::ColorType::Rgb => {
                for chunk in buf.chunks(3) {
                    pixels.push(Color::rgb(
                        chunk[0] as f32 / 255.0,
                        chunk[1] as f32 / 255.0,
                        chunk[2] as f32 / 255.0,
                    ));
                }
            }
            png::ColorType::Rgba => {
                for chunk in buf.chunks(4) {
                    pixels.push(Color::new(
                        chunk[0] as f32 / 255.0,
                        chunk[1] as f32 / 255.0,
                        chunk[2] as f32 / 255.0,
                        chunk[3] as f32 / 255.0,
                    ));
                }
            }
            png::ColorType::Grayscale => {
                for byte in buf {
                    let val = byte as f32 / 255.0;
                    pixels.push(Color::rgb(val, val, val));
                }
            }
            png::ColorType::GrayscaleAlpha => {
                for chunk in buf.chunks(2) {
                    let val = chunk[0] as f32 / 255.0;
                    pixels.push(Color::new(val, val, val, chunk[1] as f32 / 255.0));
                }
            }
            _ => return Err("Unsupported color type".into()),
        }

        Ok(Self::from_pixels(info.width, info.height, pixels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_creation() {
        let texture = Texture::new(10, 10);
        assert_eq!(texture.width, 10);
        assert_eq!(texture.height, 10);
        assert_eq!(texture.data.len(), 100);
    }

    #[test]
    fn test_texture_set_get_pixel() {
        let mut texture = Texture::new(10, 10);
        texture.set_pixel(5, 5, Color::RED);

        let pixel = texture.get_pixel(5, 5);
        assert!(pixel.is_some());
        assert_eq!(pixel.unwrap(), Color::RED);
    }

    #[test]
    fn test_texture_clear() {
        let mut texture = Texture::new(10, 10);
        texture.clear(Color::BLUE);

        for i in 0..100 {
            assert_eq!(texture.data[i], Color::BLUE);
        }
    }

    #[test]
    fn test_texture_sample() {
        let mut texture = Texture::new(10, 10);
        texture.set_pixel(5, 5, Color::GREEN);

        let sampled = texture.sample(0.5, 0.5);
        assert_eq!(sampled, Color::GREEN);
    }
}
