use engine_core::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub body_type: BodyType,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub inverse_mass: f32,
    pub restitution: f32,
    pub friction: f32,
    pub gravity_scale: f32,
    pub is_awake: bool,
}

impl RigidBody {
    pub fn new(body_type: BodyType) -> Self {
        let (mass, inverse_mass) = match body_type {
            BodyType::Static => (f32::MAX, 0.0),
            BodyType::Dynamic => (1.0, 1.0),
            BodyType::Kinematic => (f32::MAX, 0.0),
        };

        Self {
            body_type,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass,
            inverse_mass,
            restitution: 0.5,
            friction: 0.3,
            gravity_scale: 1.0,
            is_awake: true,
        }
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        if self.body_type == BodyType::Dynamic {
            self.mass = mass;
            self.inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        }
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = restitution.clamp(0.0, 1.0);
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction.clamp(0.0, 1.0);
        self
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if self.body_type == BodyType::Dynamic {
            self.acceleration += force * self.inverse_mass;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if self.body_type == BodyType::Dynamic {
            self.velocity += impulse * self.inverse_mass;
        }
    }

    pub fn integrate(&mut self, delta: f32, gravity: Vec2) {
        if self.body_type == BodyType::Static {
            self.velocity = Vec2::ZERO;
            self.acceleration = Vec2::ZERO;
            return;
        }

        if !self.is_awake {
            return;
        }

        let gravity_force = gravity * self.gravity_scale;
        self.acceleration += gravity_force;

        self.velocity += self.acceleration * delta;
        self.position += self.velocity * delta;

        self.acceleration = Vec2::ZERO;

        if self.velocity.length_squared() < 0.01 {
            self.velocity = Vec2::ZERO;
        }
    }

    pub fn wake_up(&mut self) {
        self.is_awake = true;
    }

    pub fn sleep(&mut self) {
        self.is_awake = false;
        self.velocity = Vec2::ZERO;
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::new(BodyType::Dynamic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_creation() {
        let body = RigidBody::new(BodyType::Dynamic);
        assert_eq!(body.body_type, BodyType::Dynamic);
        assert_eq!(body.mass, 1.0);
        assert_eq!(body.position, Vec2::ZERO);
    }

    #[test]
    fn test_rigid_body_static() {
        let body = RigidBody::new(BodyType::Static);
        assert_eq!(body.body_type, BodyType::Static);
        assert_eq!(body.inverse_mass, 0.0);
    }

    #[test]
    fn test_rigid_body_with_mass() {
        let body = RigidBody::new(BodyType::Dynamic).with_mass(10.0);
        assert_eq!(body.mass, 10.0);
        assert!((body.inverse_mass - 0.1).abs() < 1e-6);
    }

    #[test]
    fn test_rigid_body_apply_force() {
        let mut body = RigidBody::new(BodyType::Dynamic).with_mass(1.0);
        body.apply_force(Vec2::new(10.0, 0.0));
        assert_eq!(body.acceleration, Vec2::new(10.0, 0.0));
    }

    #[test]
    fn test_rigid_body_apply_impulse() {
        let mut body = RigidBody::new(BodyType::Dynamic).with_mass(1.0);
        body.apply_impulse(Vec2::new(5.0, 0.0));
        assert_eq!(body.velocity, Vec2::new(5.0, 0.0));
    }

    #[test]
    fn test_rigid_body_integrate() {
        let mut body = RigidBody::new(BodyType::Dynamic).with_velocity(Vec2::new(10.0, 0.0));
        body.integrate(0.1, Vec2::ZERO);
        assert_eq!(body.position, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_rigid_body_integrate_with_gravity() {
        let mut body = RigidBody::new(BodyType::Dynamic);
        body.integrate(1.0, Vec2::new(0.0, 9.8));
        assert!((body.velocity.y - 9.8).abs() < 1e-6);
    }
}
