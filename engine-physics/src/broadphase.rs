use std::collections::{HashMap, HashSet};
use engine_core::Vec2;
use crate::body::RigidBody;
use crate::collider::Collider;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellKey {
    x: i32,
    y: i32,
}

pub struct SpatialHash {
    cell_size: f32,
    grid: HashMap<CellKey, Vec<usize>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    fn world_to_cell(&self, position: Vec2) -> CellKey {
        CellKey {
            x: (position.x / self.cell_size).floor() as i32,
            y: (position.y / self.cell_size).floor() as i32,
        }
    }

    pub fn insert(&mut self, index: usize, body: &RigidBody, collider: &Collider) {
        let bounds = collider.bounds(body.position);
        let min_cell = self.world_to_cell(bounds.min);
        let max_cell = self.world_to_cell(bounds.max);

        for x in min_cell.x..=max_cell.x {
            for y in min_cell.y..=max_cell.y {
                let key = CellKey { x, y };
                self.grid.entry(key).or_insert_with(Vec::new).push(index);
            }
        }
    }

    pub fn query_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = HashSet::new();

        for cell in self.grid.values() {
            for i in 0..cell.len() {
                for j in (i + 1)..cell.len() {
                    let a = cell[i];
                    let b = cell[j];
                    if a != b {
                        let pair = if a < b { (a, b) } else { (b, a) };
                        pairs.insert(pair);
                    }
                }
            }
        }

        pairs.into_iter().collect()
    }
}

impl Default for SpatialHash {
    fn default() -> Self {
        Self::new(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyType;

    #[test]
    fn test_spatial_hash_creation() {
        let hash = SpatialHash::new(50.0);
        assert_eq!(hash.cell_size, 50.0);
    }

    #[test]
    fn test_spatial_hash_insert() {
        let mut hash = SpatialHash::new(50.0);
        let body = RigidBody::new(BodyType::Static).with_position(Vec2::new(25.0, 25.0));
        let collider = Collider::new(Vec2::new(10.0, 10.0));

        hash.insert(0, &body, &collider);
        assert!(!hash.grid.is_empty());
    }

    #[test]
    fn test_spatial_hash_query_pairs() {
        let mut hash = SpatialHash::new(50.0);

        let body_a = RigidBody::new(BodyType::Static).with_position(Vec2::new(25.0, 25.0));
        let collider_a = Collider::new(Vec2::new(10.0, 10.0));

        let body_b = RigidBody::new(BodyType::Static).with_position(Vec2::new(30.0, 30.0));
        let collider_b = Collider::new(Vec2::new(10.0, 10.0));

        hash.insert(0, &body_a, &collider_a);
        hash.insert(1, &body_b, &collider_b);

        let pairs = hash.query_pairs();
        assert!(!pairs.is_empty());
    }

    #[test]
    fn test_spatial_hash_clear() {
        let mut hash = SpatialHash::new(50.0);
        let body = RigidBody::new(BodyType::Static).with_position(Vec2::new(25.0, 25.0));
        let collider = Collider::new(Vec2::new(10.0, 10.0));

        hash.insert(0, &body, &collider);
        hash.clear();

        assert!(hash.grid.is_empty());
    }
}
