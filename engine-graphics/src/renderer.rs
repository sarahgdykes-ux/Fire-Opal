use engine_core::{Color, Vec2, Rect};
use crate::texture::Texture;

pub struct Renderer {
    framebuffer: Texture,
    clear_color: Color,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            framebuffer: Texture::new(width, height),
            clear_color: Color::BLACK,
        }
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn clear(&mut self) {
        self.framebuffer.clear(self.clear_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as u32;
        let y = y as u32;

        if x >= self.framebuffer.width || y >= self.framebuffer.height {
            return;
        }

        self.framebuffer.set_pixel(x, y, color);
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            self.set_pixel(x, y, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        let min_x = rect.min.x as i32;
        let min_y = rect.min.y as i32;
        let max_x = rect.max.x as i32;
        let max_y = rect.max.y as i32;

        self.draw_line(min_x, min_y, max_x, min_y, color);
        self.draw_line(max_x, min_y, max_x, max_y, color);
        self.draw_line(max_x, max_y, min_x, max_y, color);
        self.draw_line(min_x, max_y, min_x, min_y, color);
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let min_x = rect.min.x.max(0.0) as u32;
        let min_y = rect.min.y.max(0.0) as u32;
        let max_x = rect.max.x.min(self.framebuffer.width as f32) as u32;
        let max_y = rect.max.y.min(self.framebuffer.height as f32) as u32;

        for y in min_y..max_y {
            for x in min_x..max_x {
                self.framebuffer.set_pixel(x, y, color);
            }
        }
    }

    pub fn draw_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        self.draw_line(v0.x as i32, v0.y as i32, v1.x as i32, v1.y as i32, color);
        self.draw_line(v1.x as i32, v1.y as i32, v2.x as i32, v2.y as i32, color);
        self.draw_line(v2.x as i32, v2.y as i32, v0.x as i32, v0.y as i32, color);
    }

    pub fn fill_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        let mut vertices = [v0, v1, v2];
        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        let [v0, v1, v2] = vertices;

        if v1.y == v2.y {
            self.fill_flat_bottom_triangle(v0, v1, v2, color);
        } else if v0.y == v1.y {
            self.fill_flat_top_triangle(v0, v1, v2, color);
        } else {
            let alpha = (v1.y - v0.y) / (v2.y - v0.y);
            let vi = v0 + (v2 - v0) * alpha;
            self.fill_flat_bottom_triangle(v0, v1, vi, color);
            self.fill_flat_top_triangle(v1, vi, v2, color);
        }
    }

    fn fill_flat_bottom_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        let inv_slope1 = if v1.y != v0.y { (v1.x - v0.x) / (v1.y - v0.y) } else { 0.0 };
        let inv_slope2 = if v2.y != v0.y { (v2.x - v0.x) / (v2.y - v0.y) } else { 0.0 };

        let mut curx1 = v0.x;
        let mut curx2 = v0.x;

        for y in (v0.y as i32)..=(v1.y as i32) {
            let y = y as f32;
            let start_x = curx1.min(curx2);
            let end_x = curx1.max(curx2);

            for x in (start_x as i32)..=(end_x as i32) {
                self.set_pixel(x, y as i32, color);
            }

            curx1 += inv_slope1;
            curx2 += inv_slope2;
        }
    }

    fn fill_flat_top_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        let inv_slope1 = if v2.y != v0.y { (v2.x - v0.x) / (v2.y - v0.y) } else { 0.0 };
        let inv_slope2 = if v2.y != v1.y { (v2.x - v1.x) / (v2.y - v1.y) } else { 0.0 };

        let mut curx1 = v2.x;
        let mut curx2 = v2.x;

        for y in (v0.y as i32)..=(v2.y as i32).rev() {
            let y = y as f32;
            let start_x = curx1.min(curx2);
            let end_x = curx1.max(curx2);

            for x in (start_x as i32)..=(end_x as i32) {
                self.set_pixel(x, y as i32, color);
            }

            curx1 -= inv_slope1;
            curx2 -= inv_slope2;
        }
    }

    pub fn framebuffer(&self) -> &Texture {
        &self.framebuffer
    }

    pub fn framebuffer_mut(&mut self) -> &mut Texture {
        &mut self.framebuffer
    }

    pub fn width(&self) -> u32 {
        self.framebuffer.width
    }

    pub fn height(&self) -> u32 {
        self.framebuffer.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(800, 600);
        assert_eq!(renderer.width(), 800);
        assert_eq!(renderer.height(), 600);
    }

    #[test]
    fn test_renderer_set_pixel() {
        let mut renderer = Renderer::new(10, 10);
        renderer.set_pixel(5, 5, Color::RED);

        let pixel = renderer.framebuffer().get_pixel(5, 5);
        assert!(pixel.is_some());
        assert_eq!(pixel.unwrap(), Color::RED);
    }

    #[test]
    fn test_renderer_clear() {
        let mut renderer = Renderer::new(10, 10);
        renderer.set_clear_color(Color::BLUE);
        renderer.clear();

        let pixel = renderer.framebuffer().get_pixel(5, 5);
        assert!(pixel.is_some());
        assert_eq!(pixel.unwrap(), Color::BLUE);
    }

    #[test]
    fn test_renderer_draw_line() {
        let mut renderer = Renderer::new(100, 100);
        renderer.draw_line(0, 0, 10, 10, Color::GREEN);

        let pixel = renderer.framebuffer().get_pixel(5, 5);
        assert!(pixel.is_some());
        assert_eq!(pixel.unwrap(), Color::GREEN);
    }

    #[test]
    fn test_renderer_fill_rect() {
        let mut renderer = Renderer::new(100, 100);
        let rect = Rect::from_min_size(Vec2::new(10.0, 10.0), Vec2::new(20.0, 20.0));
        renderer.fill_rect(rect, Color::YELLOW);

        let pixel = renderer.framebuffer().get_pixel(15, 15);
        assert!(pixel.is_some());
        assert_eq!(pixel.unwrap(), Color::YELLOW);
    }
}
