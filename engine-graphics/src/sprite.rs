use engine_core::{Vec2, Mat3, Color};
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture: Texture,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub color: Color,
    pub z_index: f32,
    pub visible: bool,
}

impl Sprite {
    pub fn new(texture: Texture) -> Self {
        Self {
            texture,
            position: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
            color: Color::WHITE,
            z_index: 0.0,
            visible: true,
        }
    }

    pub fn transform(&self) -> Mat3 {
        Mat3::from_scale_translation_rotation(
            self.scale * Vec2::new(self.texture.width as f32, self.texture.height as f32),
            self.position,
            self.rotation,
        )
    }

    pub fn bounds(&self) -> engine_core::Rect {
        let half_size = Vec2::new(self.texture.width as f32, self.texture.height as f32) * self.scale * 0.5;
        engine_core::Rect::from_center(self.position, half_size * 2.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpriteBatch {
    sprites: Vec<Sprite>,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    pub fn add(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    pub fn remove(&mut self, index: usize) -> Option<Sprite> {
        if index < self.sprites.len() {
            Some(self.sprites.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&Sprite> {
        self.sprites.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Sprite> {
        self.sprites.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sprite> {
        self.sprites.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Sprite> {
        self.sprites.iter_mut()
    }

    pub fn sort_by_z_index(&mut self) {
        self.sprites.sort_by(|a, b| a.z_index.partial_cmp(&b.z_index).unwrap());
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_creation() {
        let texture = Texture::new(32, 32);
        let sprite = Sprite::new(texture);

        assert_eq!(sprite.position, Vec2::ZERO);
        assert_eq!(sprite.scale, Vec2::ONE);
        assert_eq!(sprite.rotation, 0.0);
        assert!(sprite.visible);
    }

    #[test]
    fn test_sprite_batch_add() {
        let mut batch = SpriteBatch::new();
        let texture = Texture::new(32, 32);
        let sprite = Sprite::new(texture);

        batch.add(sprite.clone());
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_sprite_batch_remove() {
        let mut batch = SpriteBatch::new();
        let texture = Texture::new(32, 32);
        let sprite = Sprite::new(texture);

        batch.add(sprite);
        let removed = batch.remove(0);
        assert!(removed.is_some());
        assert_eq!(batch.len(), 0);
    }

    #[test]
    fn test_sprite_batch_sort() {
        let mut batch = SpriteBatch::new();
        let texture = Texture::new(32, 32);

        let mut sprite1 = Sprite::new(texture.clone());
        sprite1.z_index = 2.0;

        let mut sprite2 = Sprite::new(texture.clone());
        sprite2.z_index = 1.0;

        let mut sprite3 = Sprite::new(texture);
        sprite3.z_index = 3.0;

        batch.add(sprite1);
        batch.add(sprite2);
        batch.add(sprite3);

        batch.sort_by_z_index();

        assert_eq!(batch.sprites[0].z_index, 1.0);
        assert_eq!(batch.sprites[1].z_index, 2.0);
        assert_eq!(batch.sprites[2].z_index, 3.0);
    }
}
