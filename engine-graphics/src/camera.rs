use engine_core::{Vec2, Mat3, Rect};

pub struct Camera {
    position: Vec2,
    zoom: f32,
    rotation: f32,
    viewport_size: Vec2,
}

impl Camera {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(viewport_width, viewport_height),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1);
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn viewport_size(&self) -> Vec2 {
        self.viewport_size
    }

    pub fn set_viewport_size(&mut self, size: Vec2) {
        self.viewport_size = size;
    }

    pub fn view_matrix(&self) -> Mat3 {
        let center = self.viewport_size * 0.5;
        let translate = Mat3::translation(center);
        let rotate = Mat3::rotation(-self.rotation);
        let scale = Mat3::scale(Vec2::splat(self.zoom));
        let position = Mat3::translation(-self.position);

        translate * rotate * scale * position
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let inv_view = self.view_matrix().inverse();
        inv_view.transform_point(screen_pos)
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        self.view_matrix().transform_point(world_pos)
    }

    pub fn world_bounds(&self) -> Rect {
        let top_left = self.screen_to_world(Vec2::ZERO);
        let bottom_right = self.screen_to_world(self.viewport_size);
        Rect::new(top_left, bottom_right)
    }

    pub fn is_visible(&self, world_rect: Rect) -> bool {
        let view_bounds = self.world_bounds();
        view_bounds.intersects(world_rect)
    }

    pub fn follow(&mut self, target: Vec2, lerp_factor: f32) {
        self.position = self.position.lerp(target, lerp_factor);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(800.0, 600.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(800.0, 600.0);
        assert_eq!(camera.position(), Vec2::ZERO);
        assert_eq!(camera.zoom(), 1.0);
        assert_eq!(camera.rotation(), 0.0);
    }

    #[test]
    fn test_camera_set_position() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_position(Vec2::new(100.0, 200.0));
        assert_eq!(camera.position(), Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_camera_set_zoom() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_zoom(2.0);
        assert_eq!(camera.zoom(), 2.0);
    }

    #[test]
    fn test_camera_world_to_screen() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_position(Vec2::new(400.0, 300.0));

        let screen_pos = camera.world_to_screen(Vec2::new(400.0, 300.0));
        let center = Vec2::new(400.0, 300.0);
        assert!((screen_pos.x - center.x).abs() < 1.0);
        assert!((screen_pos.y - center.y).abs() < 1.0);
    }

    #[test]
    fn test_camera_is_visible() {
        let camera = Camera::new(800.0, 600.0);
        let visible_rect = Rect::from_center(Vec2::new(400.0, 300.0), Vec2::new(100.0, 100.0));
        assert!(camera.is_visible(visible_rect));

        let far_rect = Rect::from_center(Vec2::new(5000.0, 5000.0), Vec2::new(100.0, 100.0));
        assert!(!camera.is_visible(far_rect));
    }
}
