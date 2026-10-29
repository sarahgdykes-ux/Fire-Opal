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

    /// Copies this texture into a packed `0xAARRGGBB` pixel buffer (e.g. a
    /// softbuffer surface) of size `dst_width` x `dst_height`.
    ///
    /// Only the overlapping region is copied, so a size mismatch (for example
    /// during a window resize) can never write out of bounds. Any destination
    /// pixels outside the texture are set to 0 (black).
    pub fn blit_to_u32(&self, dst: &mut [u32], dst_width: u32, dst_height: u32) {
        let dst_w = dst_width as usize;
        let dst_h = dst_height as usize;
        let src_w = self.width as usize;
        let copy_w = dst_w.min(src_w);
        let copy_h = dst_h.min(self.height as usize);

        for y in 0..dst_h {
            let row_start = y * dst_w;
            let Some(row) = dst.get_mut(row_start..row_start + dst_w) else {
                break;
            };
            if y < copy_h {
                let src_row = &self.data[y * src_w..y * src_w + copy_w];
                for (d, s) in row[..copy_w].iter_mut().zip(src_row) {
                    *d = s.to_u32();
                }
                row[copy_w..].fill(0);
            } else {
                row.fill(0);
            }
        }
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

    #[test]
    fn test_blit_to_u32_packs_argb() {
        let mut tex = Texture::new(2, 1);
        tex.set_pixel(0, 0, Color::RED);
        tex.set_pixel(1, 0, Color::BLUE);
        let mut dst = [0u32; 2];
        tex.blit_to_u32(&mut dst, 2, 1);
        assert_eq!(dst[0] & 0x00FF_FFFF, 0x00FF_0000);
        assert_eq!(dst[1] & 0x00FF_FFFF, 0x0000_00FF);
    }

    #[test]
    fn test_blit_to_u32_handles_size_mismatch() {
        let mut tex = Texture::new(4, 4);
        tex.clear(Color::WHITE);

        // destination larger than the texture: extra area is black
        let mut big = vec![0xDEAD_BEEFu32; 6 * 5];
        tex.blit_to_u32(&mut big, 6, 5);
        assert_eq!(big[0] & 0x00FF_FFFF, 0x00FF_FFFF);
        assert_eq!(big[5], 0); // x=5 is outside the texture
        assert_eq!(big[4 * 6], 0); // y=4 is outside the texture

        // destination smaller than the texture: no panic, no overrun
        let mut small = vec![0u32; 2 * 3];
        tex.blit_to_u32(&mut small, 2, 3);
        assert!(small.iter().all(|p| p & 0x00FF_FFFF == 0x00FF_FFFF));

        // destination slice shorter than claimed: must not panic
        let mut short = vec![0u32; 3];
        tex.blit_to_u32(&mut short, 4, 4);
    }
}
