use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const UP: Self = Self { x: 0.0, y: -1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self { x: self.x / len, y: self.y / len }
        } else {
            Self::ZERO
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub m: [f32; 9],
}

impl Mat3 {
    pub const IDENTITY: Self = Self {
        m: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    };

    pub fn new(m: [f32; 9]) -> Self {
        Self { m }
    }

    pub fn from_scale_translation_rotation(scale: Vec2, translation: Vec2, rotation: f32) -> Self {
        let cos = rotation.cos();
        let sin = rotation.sin();
        Self {
            m: [
                scale.x * cos,
                scale.x * sin,
                0.0,
                -scale.y * sin,
                scale.y * cos,
                0.0,
                translation.x,
                translation.y,
                1.0,
            ],
        }
    }

    pub fn translation(translation: Vec2) -> Self {
        Self {
            m: [
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, translation.x, translation.y, 1.0,
            ],
        }
    }

    pub fn scale(scale: Vec2) -> Self {
        Self {
            m: [scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            m: [cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn inverse(self) -> Self {
        let [a, b, c, d, e, f, g, h, i] = self.m;
        let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);

        if det.abs() < 1e-6 {
            return Self::IDENTITY;
        }

        let inv_det = 1.0 / det;
        Self {
            m: [
                (e * i - f * h) * inv_det,
                (c * h - b * i) * inv_det,
                (b * f - c * e) * inv_det,
                (f * g - d * i) * inv_det,
                (a * i - c * g) * inv_det,
                (c * d - a * f) * inv_det,
                (d * h - e * g) * inv_det,
                (b * g - a * h) * inv_det,
                (a * e - b * d) * inv_det,
            ],
        }
    }

    pub fn transform_point(self, point: Vec2) -> Vec2 {
        Vec2 {
            x: self.m[0] * point.x + self.m[3] * point.y + self.m[6],
            y: self.m[1] * point.x + self.m[4] * point.y + self.m[7],
        }
    }

    pub fn transform_vector(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self.m[0] * vec.x + self.m[3] * vec.y,
            y: self.m[1] * vec.x + self.m[4] * vec.y,
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let [a, b, c, d, e, f, g, h, i] = self.m;
        let [j, k, l, m, n, o, p, q, r] = other.m;

        Self {
            m: [
                a * j + d * k + g * l,
                b * j + e * k + h * l,
                c * j + f * k + i * l,
                a * m + d * n + g * o,
                b * m + e * n + h * o,
                c * m + f * n + i * o,
                a * p + d * q + g * r,
                b * p + e * q + h * r,
                c * p + f * q + i * r,
            ],
        }
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.mul(other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub const ZERO: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::ZERO,
    };

    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_min_size(min: Vec2, size: Vec2) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn from_center(center: Vec2, size: Vec2) -> Self {
        let half = size * 0.5;
        Self {
            min: center - half,
            max: center + half,
        }
    }

    pub fn size(self) -> Vec2 {
        self.max - self.min
    }

    pub fn center(self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y && point.y <= self.max.y
    }

    pub fn contains_rect(self, other: Self) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min.x < other.max.x && self.max.x > other.min.x && self.min.y < other.max.y && self.max.y > other.min.y
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        Some(Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        })
    }

    pub fn union(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn expand(self, amount: Vec2) -> Self {
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    pub fn offset(self, offset: Vec2) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn to_rgba_u8(self) -> (u8, u8, u8, u8) {
        (
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    pub fn to_u32(self) -> u32 {
        let (r, g, b, a) = self.to_rgba_u8();
        ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);
        assert_eq!(a - b, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_mul_scalar() {
        let v = Vec2::new(2.0, 3.0);
        assert_eq!(v * 2.0, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_div_scalar() {
        let v = Vec2::new(4.0, 6.0);
        assert_eq!(v / 2.0, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_vec2_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.dot(b), 11.0);
    }

    #[test]
    fn test_vec2_cross() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert_eq!(a.cross(b), 1.0);
    }

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec2_rotate() {
        let v = Vec2::new(1.0, 0.0);
        let rotated = v.rotate(std::f32::consts::PI / 2.0);
        assert!((rotated.x - 0.0).abs() < 1e-6);
        assert!((rotated.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(rect.contains(Vec2::new(5.0, 5.0)));
        assert!(!rect.contains(Vec2::new(15.0, 5.0)));
    }

    #[test]
    fn test_rect_intersects() {
        let a = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let b = Rect::from_min_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        assert!(a.intersects(b));

        let c = Rect::from_min_size(Vec2::new(20.0, 20.0), Vec2::new(10.0, 10.0));
        assert!(!a.intersects(c));
    }

    #[test]
    fn test_rect_intersection() {
        let a = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let b = Rect::from_min_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        let intersection = a.intersection(b).unwrap();
        assert_eq!(intersection.min, Vec2::new(5.0, 5.0));
        assert_eq!(intersection.max, Vec2::new(10.0, 10.0));
    }

    #[test]
    fn test_mat3_identity() {
        let m = Mat3::IDENTITY;
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(m.transform_point(v), v);
    }

    #[test]
    fn test_mat3_translation() {
        let m = Mat3::translation(Vec2::new(5.0, 10.0));
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(m.transform_point(v), Vec2::new(6.0, 12.0));
    }

    #[test]
    fn test_mat3_scale() {
        let m = Mat3::scale(Vec2::new(2.0, 3.0));
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(m.transform_point(v), Vec2::new(2.0, 6.0));
    }

    #[test]
    fn test_mat3_rotation() {
        let m = Mat3::rotation(std::f32::consts::PI / 2.0);
        let v = Vec2::new(1.0, 0.0);
        let result = m.transform_point(v);
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_color_to_u32() {
        let c = Color::WHITE;
        assert_eq!(c.to_u32(), 0xFFFFFFFF);
    }

    #[test]
    fn test_color_from_rgba_u8() {
        let c = Color::from_rgba_u8(255, 128, 64, 255);
        assert!((c.r - 1.0).abs() < 1e-6);
        assert!((c.g - 0.50196).abs() < 1e-5);
        assert!((c.b - 0.25098).abs() < 1e-5);
        assert_eq!(c.a, 1.0);
    }
}
