use engine_core::Vec2;
use crate::body::RigidBody;
use crate::collider::Collider;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision {
    pub normal: Vec2,
    pub penetration: f32,
    pub has_collision: bool,
}

impl Collision {
    pub const NONE: Self = Self {
        normal: Vec2::ZERO,
        penetration: 0.0,
        has_collision: false,
    };
}

impl Default for Collision {
    fn default() -> Self {
        Self::NONE
    }
}

pub fn detect_aabb_collision(
    body_a: &RigidBody,
    collider_a: &Collider,
    body_b: &RigidBody,
    collider_b: &Collider,
) -> Collision {
    let bounds_a = collider_a.bounds(body_a.position);
    let bounds_b = collider_b.bounds(body_b.position);

    if !bounds_a.intersects(bounds_b) {
        return Collision::NONE;
    }

    let intersection = bounds_a.intersection(bounds_b).unwrap();
    let penetration = intersection.size();

    let center_a = bounds_a.center();
    let center_b = bounds_b.center();
    let diff = center_a - center_b;

    let (normal, pen_depth) = if penetration.x < penetration.y {
        if diff.x > 0.0 {
            (Vec2::new(1.0, 0.0), penetration.x)
        } else {
            (Vec2::new(-1.0, 0.0), penetration.x)
        }
    } else {
        if diff.y > 0.0 {
            (Vec2::new(0.0, 1.0), penetration.y)
        } else {
            (Vec2::new(0.0, -1.0), penetration.y)
        }
    };

    Collision {
        normal,
        penetration: pen_depth,
        has_collision: true,
    }
}

pub fn resolve_collision(
    body_a: &mut RigidBody,
    body_b: &mut RigidBody,
    collision: &Collision,
) {
    if !collision.has_collision {
        return;
    }

    if body_a.body_type == crate::body::BodyType::Static
        && body_b.body_type == crate::body::BodyType::Static
    {
        return;
    }

    let total_inverse_mass = body_a.inverse_mass + body_b.inverse_mass;

    if total_inverse_mass == 0.0 {
        return;
    }

    let move_per_inv_mass = collision.penetration / total_inverse_mass;

    // `collision.normal` points from body_b toward body_a (see detect_aabb_collision:
    // it is derived from `center_a - center_b`). To separate the bodies, body_a must
    // move further along the normal and body_b must move further against it.
    if body_a.body_type != crate::body::BodyType::Static {
        body_a.position += collision.normal * (move_per_inv_mass * body_a.inverse_mass);
    }

    if body_b.body_type != crate::body::BodyType::Static {
        body_b.position -= collision.normal * (move_per_inv_mass * body_b.inverse_mass);
    }

    let relative_velocity = body_b.velocity - body_a.velocity;
    let velocity_along_normal = relative_velocity.dot(collision.normal);

    // If the relative velocity along the normal is already positive, the bodies are
    // separating (moving apart) and no impulse should be applied.
    if velocity_along_normal < 0.0 {
        return;
    }

    let restitution = body_a.restitution.min(body_b.restitution);
    let j = -(1.0 + restitution) * velocity_along_normal;
    let j = j / total_inverse_mass;

    let impulse = collision.normal * j;

    if body_a.body_type == crate::body::BodyType::Dynamic {
        body_a.velocity -= impulse * body_a.inverse_mass;
    }

    if body_b.body_type == crate::body::BodyType::Dynamic {
        body_b.velocity += impulse * body_b.inverse_mass;
    }

    let tangent = relative_velocity - (collision.normal * velocity_along_normal);
    let tangent = if tangent.length_squared() > 1e-6 {
        tangent.normalize()
    } else {
        Vec2::ZERO
    };

    let jt = -relative_velocity.dot(tangent);
    let jt = jt / total_inverse_mass;

    let friction = body_a.friction.min(body_b.friction);

    let friction_impulse = if jt.abs() < j * friction {
        tangent * jt
    } else {
        tangent * (-j * friction)
    };

    if body_a.body_type == crate::body::BodyType::Dynamic {
        body_a.velocity -= friction_impulse * body_a.inverse_mass;
    }

    if body_b.body_type == crate::body::BodyType::Dynamic {
        body_b.velocity += friction_impulse * body_b.inverse_mass;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyType;

    #[test]
    fn test_aabb_collision_detection() {
        let body_a = RigidBody::new(BodyType::Static).with_position(Vec2::new(0.0, 0.0));
        let collider_a = Collider::new(Vec2::new(10.0, 10.0));

        let body_b = RigidBody::new(BodyType::Static).with_position(Vec2::new(5.0, 5.0));
        let collider_b = Collider::new(Vec2::new(10.0, 10.0));

        let collision = detect_aabb_collision(&body_a, &collider_a, &body_b, &collider_b);
        assert!(collision.has_collision);
    }

    #[test]
    fn test_aabb_no_collision() {
        let body_a = RigidBody::new(BodyType::Static).with_position(Vec2::new(0.0, 0.0));
        let collider_a = Collider::new(Vec2::new(10.0, 10.0));

        let body_b = RigidBody::new(BodyType::Static).with_position(Vec2::new(20.0, 20.0));
        let collider_b = Collider::new(Vec2::new(10.0, 10.0));

        let collision = detect_aabb_collision(&body_a, &collider_a, &body_b, &collider_b);
        assert!(!collision.has_collision);
    }

    #[test]
    fn test_collision_resolution() {
        let mut body_a = RigidBody::new(BodyType::Dynamic)
            .with_position(Vec2::new(0.0, 0.0))
            .with_velocity(Vec2::new(10.0, 0.0));
        let collider_a = Collider::new(Vec2::new(10.0, 10.0));

        let mut body_b = RigidBody::new(BodyType::Dynamic)
            .with_position(Vec2::new(5.0, 0.0))
            .with_velocity(Vec2::new(-10.0, 0.0));
        let collider_b = Collider::new(Vec2::new(10.0, 10.0));

        let collision = detect_aabb_collision(&body_a, &collider_a, &body_b, &collider_b);
        resolve_collision(&mut body_a, &mut body_b, &collision);

        assert!(body_a.velocity.x < 10.0);
        assert!(body_b.velocity.x > -10.0);
    }
}
