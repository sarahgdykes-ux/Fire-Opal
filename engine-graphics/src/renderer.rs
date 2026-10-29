use engine_core::{Color, Vec2, Rect};
use crate::camera::Camera;
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

    /// Resizes the framebuffer. Contents are discarded (the next `clear()`
    /// repaints it). Does nothing if the size is unchanged.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == self.framebuffer.width && height == self.framebuffer.height {
            return;
        }
        self.framebuffer = Texture::new(width, height);
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
        // f32::min/max ignore NaN, which would turn a NaN rect into a full-screen fill.
        if !(rect.min.x.is_finite()
            && rect.min.y.is_finite()
            && rect.max.x.is_finite()
            && rect.max.y.is_finite())
        {
            return;
        }

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

        for y in ((v0.y as i32)..=(v2.y as i32)).rev() {
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

    // ------------------------------------------------------------------
    // World-space drawing
    //
    // The methods above work in *screen* pixels (use them for UI). The
    // `*_world` methods below take a `Camera` and work in *world* units, so
    // position, zoom and rotation of the camera are applied for you.
    // ------------------------------------------------------------------

    /// Draws a line between two world-space points. The line is clipped to the
    /// framebuffer first, so far-off-screen endpoints are cheap and safe.
    pub fn draw_line_world(&mut self, camera: &Camera, a: Vec2, b: Vec2, color: Color) {
        let a = camera.world_to_screen(a);
        let b = camera.world_to_screen(b);
        let w = self.framebuffer.width as f32;
        let h = self.framebuffer.height as f32;

        if let Some((a, b)) = clip_line(a, b, w, h) {
            self.draw_line(
                a.x.round() as i32,
                a.y.round() as i32,
                b.x.round() as i32,
                b.y.round() as i32,
                color,
            );
        }
    }

    /// Fills a world-space rectangle. With an unrotated camera this is a fast
    /// axis-aligned fill; with a rotated camera each covered pixel is mapped
    /// back into the world and tested, so the result is exact (no seams).
    pub fn fill_rect_world(&mut self, camera: &Camera, rect: Rect, color: Color) {
        let min = Vec2::new(rect.min.x.min(rect.max.x), rect.min.y.min(rect.max.y));
        let max = Vec2::new(rect.min.x.max(rect.max.x), rect.min.y.max(rect.max.y));

        if camera.rotation() == 0.0 {
            let a = camera.world_to_screen(min);
            let b = camera.world_to_screen(max);
            let screen = Rect::new(
                Vec2::new(a.x.min(b.x).round(), a.y.min(b.y).round()),
                Vec2::new(a.x.max(b.x).round(), a.y.max(b.y).round()),
            );
            self.fill_rect(screen, color);
            return;
        }

        let corners = [
            camera.world_to_screen(min),
            camera.world_to_screen(Vec2::new(max.x, min.y)),
            camera.world_to_screen(max),
            camera.world_to_screen(Vec2::new(min.x, max.y)),
        ];
        let mut lo = corners[0];
        let mut hi = corners[0];
        for c in &corners[1..] {
            lo = Vec2::new(lo.x.min(c.x), lo.y.min(c.y));
            hi = Vec2::new(hi.x.max(c.x), hi.y.max(c.y));
        }

        // Only visit pixels inside both the rotated rect's bounding box and the framebuffer.
        let x0 = lo.x.floor().max(0.0) as u32;
        let y0 = lo.y.floor().max(0.0) as u32;
        let x1 = (hi.x.ceil().max(0.0) as u32).min(self.framebuffer.width);
        let y1 = (hi.y.ceil().max(0.0) as u32).min(self.framebuffer.height);

        let inv = camera.view_matrix().inverse();
        for y in y0..y1 {
            for x in x0..x1 {
                let p = inv.transform_point(Vec2::new(x as f32 + 0.5, y as f32 + 0.5));
                if p.x >= min.x && p.x < max.x && p.y >= min.y && p.y < max.y {
                    self.framebuffer.set_pixel(x, y, color);
                }
            }
        }
    }

    /// Outlines a world-space rectangle.
    pub fn draw_rect_world(&mut self, camera: &Camera, rect: Rect, color: Color) {
        let tl = rect.min;
        let tr = Vec2::new(rect.max.x, rect.min.y);
        let br = rect.max;
        let bl = Vec2::new(rect.min.x, rect.max.y);
        self.draw_line_world(camera, tl, tr, color);
        self.draw_line_world(camera, tr, br, color);
        self.draw_line_world(camera, br, bl, color);
        self.draw_line_world(camera, bl, tl, color);
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

/// Liang-Barsky clip of the segment `a`-`b` to the pixel area
/// `[0, w-1] x [0, h-1]`. Returns `None` if nothing is visible (or the input
/// is not finite).
fn clip_line(a: Vec2, b: Vec2, w: f32, h: f32) -> Option<(Vec2, Vec2)> {
    if w < 1.0 || h < 1.0 {
        return None;
    }
    if !(a.x.is_finite() && a.y.is_finite() && b.x.is_finite() && b.y.is_finite()) {
        return None;
    }

    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let mut t0 = 0.0f32;
    let mut t1 = 1.0f32;

    for (p, q) in [
        (-dx, a.x),
        (dx, (w - 1.0) - a.x),
        (-dy, a.y),
        (dy, (h - 1.0) - a.y),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None; // parallel to this edge and outside it
            }
        } else {
            let t = q / p;
            if p < 0.0 {
                if t > t1 {
                    return None;
                }
                t0 = t0.max(t);
            } else {
                if t < t0 {
                    return None;
                }
                t1 = t1.min(t);
            }
        }
    }

    Some((
        Vec2::new(a.x + dx * t0, a.y + dy * t0),
        Vec2::new(a.x + dx * t1, a.y + dy * t1),
    ))
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

    #[test]
    fn test_renderer_resize() {
        let mut renderer = Renderer::new(10, 10);
        renderer.resize(20, 5);
        assert_eq!(renderer.width(), 20);
        assert_eq!(renderer.height(), 5);
        assert_eq!(renderer.framebuffer().data.len(), 100);

        // drawing outside the new bounds is still safely clipped
        renderer.set_pixel(15, 7, Color::RED);
        renderer.fill_rect(Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)), Color::RED);
    }

    fn cam_at(pos: Vec2) -> Camera {
        let mut cam = Camera::new(100.0, 100.0);
        cam.set_position(pos);
        cam
    }

    fn count(renderer: &Renderer, color: Color) -> usize {
        renderer.framebuffer().data.iter().filter(|p| **p == color).count()
    }

    fn square(center: Vec2, w: f32, h: f32) -> Rect {
        Rect::from_center(center, Vec2::new(w, h))
    }

    #[test]
    fn test_world_rect_follows_camera() {
        let mut r = Renderer::new(100, 100);
        r.set_clear_color(Color::BLACK);
        r.clear();
        let cam = cam_at(Vec2::new(1000.0, 1000.0));
        r.fill_rect_world(&cam, square(Vec2::new(1000.0, 1000.0), 10.0, 10.0), Color::RED);

        assert_eq!(r.framebuffer().get_pixel(50, 50), Some(Color::RED));
        assert_eq!(r.framebuffer().get_pixel(10, 10), Some(Color::BLACK));
        assert_eq!(count(&r, Color::RED), 100);
    }

    #[test]
    fn test_world_rect_zoom_scales_area() {
        let mut r = Renderer::new(100, 100);
        r.clear();
        let mut cam = cam_at(Vec2::new(0.0, 0.0));
        cam.set_zoom(2.0);
        r.fill_rect_world(&cam, square(Vec2::ZERO, 10.0, 10.0), Color::RED);
        assert_eq!(count(&r, Color::RED), 400);
    }

    #[test]
    fn test_world_rect_rotated_quarter_turn_swaps_axes() {
        let mut r = Renderer::new(100, 100);
        r.clear();
        let mut cam = cam_at(Vec2::ZERO);
        cam.set_rotation(std::f32::consts::FRAC_PI_2);
        // 20 wide x 10 tall in the world -> 10 wide x 20 tall on screen
        r.fill_rect_world(&cam, square(Vec2::ZERO, 20.0, 10.0), Color::RED);

        assert_eq!(r.framebuffer().get_pixel(50, 58), Some(Color::RED));
        assert_eq!(r.framebuffer().get_pixel(58, 50), Some(Color::BLACK));
        assert_eq!(count(&r, Color::RED), 200);
    }

    #[test]
    fn test_world_rect_rotated_45_degrees_keeps_area() {
        let mut r = Renderer::new(100, 100);
        r.clear();
        let mut cam = cam_at(Vec2::ZERO);
        cam.set_rotation(std::f32::consts::FRAC_PI_4);
        r.fill_rect_world(&cam, square(Vec2::ZERO, 20.0, 10.0), Color::RED);
        let n = count(&r, Color::RED) as f32;
        assert!((n - 200.0).abs() < 20.0, "area was {}", n);
    }

    #[test]
    fn test_world_drawing_far_offscreen_is_safe_and_fast() {
        let mut r = Renderer::new(100, 100);
        r.clear();
        let cam = cam_at(Vec2::ZERO);

        // diagonal through the screen from absurdly far away: must not overflow or hang
        r.draw_line_world(&cam, Vec2::new(-1.0e9, -1.0e9), Vec2::new(1.0e9, 1.0e9), Color::GREEN);
        assert!(count(&r, Color::GREEN) > 50);

        // entirely off screen / non-finite: draws nothing, does not panic
        r.draw_line_world(&cam, Vec2::new(1.0e9, 1.0e9), Vec2::new(2.0e9, 1.0e9), Color::BLUE);
        r.draw_line_world(&cam, Vec2::new(f32::NAN, 0.0), Vec2::new(1.0, 1.0), Color::BLUE);
        r.fill_rect_world(&cam, square(Vec2::new(f32::NAN, 0.0), 10.0, 10.0), Color::BLUE);
        r.fill_rect_world(&cam, square(Vec2::new(1.0e12, 0.0), 10.0, 10.0), Color::BLUE);
        assert_eq!(count(&r, Color::BLUE), 0);
    }

    #[test]
    fn test_clip_line() {
        // fully outside
        assert!(clip_line(Vec2::new(-10.0, -10.0), Vec2::new(-1.0, -1.0), 100.0, 100.0).is_none());
        // fully inside is unchanged
        let (a, b) = clip_line(Vec2::new(10.0, 10.0), Vec2::new(20.0, 30.0), 100.0, 100.0).unwrap();
        assert_eq!((a, b), (Vec2::new(10.0, 10.0), Vec2::new(20.0, 30.0)));
        // crossing: endpoints land on the boundary
        let (a, b) = clip_line(Vec2::new(-50.0, 50.0), Vec2::new(150.0, 50.0), 100.0, 100.0).unwrap();
        assert!((a.x - 0.0).abs() < 1e-3 && (b.x - 99.0).abs() < 1e-3);
    }

    #[test]
    fn test_world_outline_draws_four_edges() {
        let mut r = Renderer::new(100, 100);
        r.clear();
        let cam = cam_at(Vec2::ZERO);
        r.draw_rect_world(&cam, square(Vec2::ZERO, 20.0, 20.0), Color::GREEN);
        assert_eq!(r.framebuffer().get_pixel(50, 40), Some(Color::GREEN)); // top edge
        assert_eq!(r.framebuffer().get_pixel(50, 50), Some(Color::BLACK)); // interior untouched
    }

    #[test]
    fn test_fill_rect_ignores_non_finite_rects() {
        let mut r = Renderer::new(20, 20);
        r.clear();
        let nan = f32::NAN;
        r.fill_rect(Rect::new(Vec2::new(nan, 0.0), Vec2::new(5.0, 5.0)), Color::RED);
        r.fill_rect(Rect::new(Vec2::new(0.0, 0.0), Vec2::new(f32::INFINITY, 5.0)), Color::RED);
        assert_eq!(count(&r, Color::RED), 0);
    }
}
