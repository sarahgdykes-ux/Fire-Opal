use engine_core::{Vec2, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColliderType {
    Solid,
    Trigger,
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub offset: Vec2,
    pub size: Vec2,
    pub collider_type: ColliderType,
    pub collision_layer: u32,
    pub collision_mask: u32,
}

impl Collider {
    pub fn new(size: Vec2) -> Self {
        Self {
            offset: Vec2::ZERO,
            size,
            collider_type: ColliderType::Solid,
            collision_layer: 1,
            collision_mask: 0xFFFFFFFF,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_type(mut self, collider_type: ColliderType) -> Self {
        self.collider_type = collider_type;
        self
    }

    pub fn with_layer(mut self, layer: u32) -> Self {
        self.collision_layer = layer;
        self
    }

    pub fn with_mask(mut self, mask: u32) -> Self {
        self.collision_mask = mask;
        self
    }

    pub fn bounds(&self, body_position: Vec2) -> Rect {
        let center = body_position + self.offset;
        Rect::from_center(center, self.size)
    }

    pub fn should_collide(&self, other: &Collider) -> bool {
        (self.collision_layer & other.collision_mask) != 0
            && (other.collision_layer & self.collision_mask) != 0
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self::new(Vec2::ONE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collider_creation() {
        let collider = Collider::new(Vec2::new(10.0, 20.0));
        assert_eq!(collider.size, Vec2::new(10.0, 20.0));
        assert_eq!(collider.offset, Vec2::ZERO);
    }

    #[test]
    fn test_collider_with_offset() {
        let collider = Collider::new(Vec2::ONE).with_offset(Vec2::new(5.0, 10.0));
        assert_eq!(collider.offset, Vec2::new(5.0, 10.0));
    }

    #[test]
    fn test_collider_bounds() {
        let collider = Collider::new(Vec2::new(10.0, 20.0));
        let bounds = collider.bounds(Vec2::ZERO);
        assert_eq!(bounds.size(), Vec2::new(10.0, 20.0));
        assert_eq!(bounds.center(), Vec2::ZERO);
    }

    #[test]
    fn test_collider_should_collide() {
        let a = Collider::new(Vec2::ONE).with_layer(1).with_mask(2);
        let b = Collider::new(Vec2::ONE).with_layer(2).with_mask(1);
        assert!(a.should_collide(&b));
    }

    #[test]
    fn test_collider_should_not_collide() {
        let a = Collider::new(Vec2::ONE).with_layer(1).with_mask(1);
        let b = Collider::new(Vec2::ONE).with_layer(2).with_mask(2);
        assert!(!a.should_collide(&b));
    }
}
