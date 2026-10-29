use engine_core::{Vec2, Mat3, Rect};

#[derive(Debug, Clone)]
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

    /// Axis-aligned bounding box (in world space) of everything the camera can
    /// see. When the camera is rotated the view is a rotated rectangle, so all
    /// four screen corners are used, not just two opposite ones.
    pub fn world_bounds(&self) -> Rect {
        let inv_view = self.view_matrix().inverse();
        let w = self.viewport_size.x;
        let h = self.viewport_size.y;
        let corners = [
            inv_view.transform_point(Vec2::new(0.0, 0.0)),
            inv_view.transform_point(Vec2::new(w, 0.0)),
            inv_view.transform_point(Vec2::new(w, h)),
            inv_view.transform_point(Vec2::new(0.0, h)),
        ];

        let mut min = corners[0];
        let mut max = corners[0];
        for c in &corners[1..] {
            min = Vec2::new(min.x.min(c.x), min.y.min(c.y));
            max = Vec2::new(max.x.max(c.x), max.y.max(c.y));
        }
        Rect::new(min, max)
    }

    pub fn is_visible(&self, world_rect: Rect) -> bool {
        let view_bounds = self.world_bounds();
        view_bounds.intersects(world_rect)
    }

    /// Moves a fixed fraction of the way to `target` per call. The speed depends
    /// on how often it is called; prefer `follow_smooth` for per-frame use.
    pub fn follow(&mut self, target: Vec2, lerp_factor: f32) {
        self.position = self.position.lerp(target, lerp_factor);
    }

    /// Frame-rate independent smooth follow. `speed` is roughly "how fast it
    /// catches up" (higher = snappier; ~5-10 feels natural); `dt` is the frame
    /// time in seconds.
    pub fn follow_smooth(&mut self, target: Vec2, speed: f32, dt: f32) {
        let t = 1.0 - (-speed.max(0.0) * dt.max(0.0)).exp();
        self.position = self.position.lerp(target, t);
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

    fn approx(a: Vec2, b: Vec2) -> bool {
        (a.x - b.x).abs() < 1e-2 && (a.y - b.y).abs() < 1e-2
    }

    #[test]
    fn test_camera_position_maps_to_viewport_center() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_position(Vec2::new(1234.0, -567.0));
        camera.set_zoom(2.5);
        camera.set_rotation(0.7);
        assert!(approx(camera.world_to_screen(camera.position()), Vec2::new(400.0, 300.0)));
    }

    #[test]
    fn test_camera_zoom_scales_distances() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_zoom(2.0);
        let a = camera.world_to_screen(Vec2::new(0.0, 0.0));
        let b = camera.world_to_screen(Vec2::new(10.0, 0.0));
        assert!(((b.x - a.x) - 20.0).abs() < 1e-3);
    }

    #[test]
    fn test_camera_round_trip_with_rotation_and_zoom() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_position(Vec2::new(300.0, 120.0));
        camera.set_zoom(1.7);
        camera.set_rotation(0.9);
        let p = Vec2::new(-42.0, 77.0);
        assert!(approx(camera.screen_to_world(camera.world_to_screen(p)), p));
    }

    #[test]
    fn test_world_bounds_contain_all_screen_corners_when_rotated() {
        let mut camera = Camera::new(800.0, 600.0);
        camera.set_position(Vec2::new(100.0, 100.0));
        camera.set_rotation(0.6);
        let bounds = camera.world_bounds();
        for corner in [
            Vec2::new(0.0, 0.0),
            Vec2::new(800.0, 0.0),
            Vec2::new(800.0, 600.0),
            Vec2::new(0.0, 600.0),
        ] {
            let w = camera.screen_to_world(corner);
            assert!(w.x >= bounds.min.x - 1e-2 && w.x <= bounds.max.x + 1e-2);
            assert!(w.y >= bounds.min.y - 1e-2 && w.y <= bounds.max.y + 1e-2);
        }
    }

    #[test]
    fn test_follow_smooth_is_frame_rate_independent() {
        let target = Vec2::new(100.0, 0.0);

        let mut one_step = Camera::new(800.0, 600.0);
        one_step.follow_smooth(target, 8.0, 1.0 / 30.0);

        let mut two_steps = Camera::new(800.0, 600.0);
        two_steps.follow_smooth(target, 8.0, 1.0 / 60.0);
        two_steps.follow_smooth(target, 8.0, 1.0 / 60.0);

        assert!((one_step.position().x - two_steps.position().x).abs() < 1e-3);
        assert!(one_step.position().x > 0.0 && one_step.position().x < 100.0);
    }
}
