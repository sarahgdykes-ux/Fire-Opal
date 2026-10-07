use engine_core::Vec2;
use crate::texture::Texture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayMode {
    Once,
    Loop,
    PingPong,
}

#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub texture: Texture,
    pub duration: f32,
    pub offset: Vec2,
}

impl AnimationFrame {
    pub fn new(texture: Texture, duration: f32) -> Self {
        Self {
            texture,
            duration,
            offset: Vec2::ZERO,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub play_mode: AnimationPlayMode,
    pub current_frame: usize,
    pub time_accumulated: f32,
    pub is_playing: bool,
    pub is_finished: bool,
    pub direction: i32,
}

impl Animation {
    pub fn new(frames: Vec<AnimationFrame>, play_mode: AnimationPlayMode) -> Self {
        Self {
            frames,
            play_mode,
            current_frame: 0,
            time_accumulated: 0.0,
            is_playing: true,
            is_finished: false,
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.is_playing || self.frames.is_empty() {
            return;
        }

        self.time_accumulated += delta;

        let current_frame = &self.frames[self.current_frame];
        
        while self.time_accumulated >= current_frame.duration {
            self.time_accumulated -= current_frame.duration;
            self.advance_frame();
        }
    }

    fn advance_frame(&mut self) {
        let next_frame = (self.current_frame as i32 + self.direction) as usize;

        match self.play_mode {
            AnimationPlayMode::Once => {
                if next_frame >= self.frames.len() {
                    self.current_frame = self.frames.len() - 1;
                    self.is_finished = true;
                    self.is_playing = false;
                } else {
                    self.current_frame = next_frame;
                }
            }
            AnimationPlayMode::Loop => {
                self.current_frame = next_frame % self.frames.len();
            }
            AnimationPlayMode::PingPong => {
                if next_frame >= self.frames.len() {
                    self.direction = -1;
                    self.current_frame = self.frames.len() - 1;
                } else if self.direction == -1 && self.current_frame == 0 {
                    self.direction = 1;
                    self.current_frame = 0;
                } else {
                    self.current_frame = next_frame;
                }
            }
        }
    }

    pub fn current_frame(&self) -> Option<&AnimationFrame> {
        self.frames.get(self.current_frame)
    }

    pub fn current_texture(&self) -> Option<&Texture> {
        self.current_frame().map(|frame| &frame.texture)
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        self.is_finished = false;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_frame = 0;
        self.time_accumulated = 0.0;
        self.is_finished = false;
        self.direction = 1;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.time_accumulated = 0.0;
        self.is_finished = false;
        self.direction = 1;
    }

    pub fn set_frame(&mut self, frame: usize) {
        if frame < self.frames.len() {
            self.current_frame = frame;
            self.time_accumulated = 0.0;
        }
    }

    pub fn duration(&self) -> f32 {
        self.frames.iter().map(|f| f.duration).sum()
    }

    pub fn progress(&self) -> f32 {
        if self.frames.is_empty() {
            return 0.0;
        }
        
        let total_time = self.duration();
        if total_time == 0.0 {
            return 0.0;
        }

        let elapsed = self.current_frame as f32 * self.frames[0].duration + self.time_accumulated;
        (elapsed / total_time).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_creation() {
        let texture = Texture::new(32, 32);
        let frame = AnimationFrame::new(texture, 0.1);
        let animation = Animation::new(vec![frame], AnimationPlayMode::Loop);
        
        assert_eq!(animation.frames.len(), 1);
        assert!(animation.is_playing);
    }

    #[test]
    fn test_animation_update() {
        let texture = Texture::new(32, 32);
        let frame = AnimationFrame::new(texture, 0.1);
        let mut animation = Animation::new(vec![frame], AnimationPlayMode::Loop);
        
        animation.update(0.05);
        assert_eq!(animation.time_accumulated, 0.05);
    }

    #[test]
    fn test_animation_play_pause() {
        let texture = Texture::new(32, 32);
        let frame = AnimationFrame::new(texture, 0.1);
        let mut animation = Animation::new(vec![frame], AnimationPlayMode::Loop);
        
        animation.pause();
        assert!(!animation.is_playing);
        
        animation.play();
        assert!(animation.is_playing);
    }

    #[test]
    fn test_animation_reset() {
        let texture = Texture::new(32, 32);
        let frame = AnimationFrame::new(texture, 0.1);
        let mut animation = Animation::new(vec![frame], AnimationPlayMode::Loop);
        
        animation.update(0.15);
        animation.reset();
        
        assert_eq!(animation.current_frame, 0);
        assert_eq!(animation.time_accumulated, 0.0);
    }
}
